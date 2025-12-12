//! Reactive State System for GlassUI
//! 
//! Provides observable values with automatic UI updates, similar to Delphi's
//! data binding but with modern reactive patterns.
//!
//! # Example
//! ```rust
//! let count = State::new(0);
//! count.subscribe(|v| println!("Count changed to: {}", v));
//! count.set(5); // Triggers subscriber
//! ```

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Unique ID generator for subscriptions
static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

fn next_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

/// A subscription handle that automatically unsubscribes when dropped
pub struct Subscription {
    id: usize,
    unsubscribe: Option<Box<dyn FnOnce()>>,
}

impl Subscription {
    /// Create a new subscription with cleanup function
    pub fn new(id: usize, unsubscribe: impl FnOnce() + 'static) -> Self {
        Self {
            id,
            unsubscribe: Some(Box::new(unsubscribe)),
        }
    }
    
    /// Keep subscription alive (prevent auto-unsubscribe)
    pub fn forget(mut self) {
        self.unsubscribe = None;
    }
    
    /// Get the subscription ID
    pub fn id(&self) -> usize {
        self.id
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if let Some(unsub) = self.unsubscribe.take() {
            unsub();
        }
    }
}

/// Internal state storage
struct StateInner<T> {
    value: T,
    subscribers: Vec<(usize, Box<dyn Fn(&T)>)>,
}

/// Observable reactive value
/// 
/// Notifies all subscribers when the value changes. This is the foundation
/// for data binding in the GlassUI framework.
pub struct State<T> {
    inner: Rc<RefCell<StateInner<T>>>,
}

impl<T: Clone + 'static> State<T> {
    /// Create a new State with initial value
    pub fn new(value: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(StateInner {
                value,
                subscribers: Vec::new(),
            })),
        }
    }
    
    /// Get the current value
    pub fn get(&self) -> T {
        self.inner.borrow().value.clone()
    }
    
    /// Set a new value and notify all subscribers
    pub fn set(&self, value: T) {
        let mut inner = self.inner.borrow_mut();
        inner.value = value;
        let value_ref = &inner.value;
        for (_, subscriber) in &inner.subscribers {
            subscriber(value_ref);
        }
    }
    
    /// Update value using a function
    pub fn update(&self, f: impl FnOnce(&T) -> T) {
        let new_value = f(&self.inner.borrow().value);
        self.set(new_value);
    }
    
    /// Subscribe to value changes
    /// 
    /// Returns a Subscription that will automatically unsubscribe when dropped.
    /// Call `.forget()` on the subscription to keep it alive indefinitely.
    pub fn subscribe(&self, f: impl Fn(&T) + 'static) -> Subscription {
        let id = next_id();
        self.inner.borrow_mut().subscribers.push((id, Box::new(f)));
        
        let inner = Rc::clone(&self.inner);
        Subscription::new(id, move || {
            inner.borrow_mut().subscribers.retain(|(sub_id, _)| *sub_id != id);
        })
    }
    
    /// Subscribe and immediately call with current value
    pub fn subscribe_immediate(&self, f: impl Fn(&T) + 'static) -> Subscription {
        f(&self.inner.borrow().value);
        self.subscribe(f)
    }
    
    /// Create a clone that shares the same underlying state
    pub fn share(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<T: Clone + 'static> Clone for State<T> {
    fn clone(&self) -> Self {
        self.share()
    }
}

impl<T: Default + Clone + 'static> Default for State<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// Computed value that derives from other State values
/// 
/// Automatically updates when dependencies change.
pub struct Computed<T> {
    state: State<T>,
    #[allow(dead_code)]
    subscriptions: Vec<Subscription>,
}

impl<T: Clone + 'static> Computed<T> {
    /// Create a computed value from a single dependency
    pub fn from<S: Clone + 'static>(
        source: &State<S>,
        compute: impl Fn(&S) -> T + 'static,
    ) -> Self {
        let initial = compute(&source.get());
        let state = State::new(initial);
        
        let state_clone = state.share();
        let sub = source.subscribe(move |v| {
            state_clone.set(compute(v));
        });
        
        Self {
            state,
            subscriptions: vec![sub],
        }
    }
    
    /// Create a computed value from two dependencies
    pub fn from2<S1: Clone + 'static, S2: Clone + 'static>(
        source1: &State<S1>,
        source2: &State<S2>,
        compute: impl Fn(&S1, &S2) -> T + Clone + 'static,
    ) -> Self {
        let initial = compute(&source1.get(), &source2.get());
        let state = State::new(initial);
        
        let state1 = state.share();
        let s2 = source2.share();
        let compute1 = compute.clone();
        let sub1 = source1.subscribe(move |v1| {
            state1.set(compute1(v1, &s2.get()));
        });
        
        let state2 = state.share();
        let s1 = source1.share();
        let sub2 = source2.subscribe(move |v2| {
            state2.set(compute(&s1.get(), v2));
        });
        
        Self {
            state,
            subscriptions: vec![sub1, sub2],
        }
    }
    
    /// Get the current computed value
    pub fn get(&self) -> T {
        self.state.get()
    }
    
    /// Subscribe to computed value changes
    pub fn subscribe(&self, f: impl Fn(&T) + 'static) -> Subscription {
        self.state.subscribe(f)
    }
}

/// Two-way binding for form controls
/// 
/// Used by widgets like TextInput, Slider, Checkbox to sync with a State value.
pub struct Binding<T> {
    state: State<T>,
}

impl<T: Clone + 'static> Binding<T> {
    /// Create a binding from a State
    pub fn new(state: &State<T>) -> Self {
        Self {
            state: state.share(),
        }
    }
    
    /// Get the current value
    pub fn get(&self) -> T {
        self.state.get()
    }
    
    /// Set the value (typically called by the widget when user interacts)
    pub fn set(&self, value: T) {
        self.state.set(value);
    }
    
    /// Subscribe to changes (typically called by the widget to update its display)
    pub fn subscribe(&self, f: impl Fn(&T) + 'static) -> Subscription {
        self.state.subscribe(f)
    }
    
    /// Get the underlying state
    pub fn state(&self) -> State<T> {
        self.state.share()
    }
}

impl<T: Clone + 'static> Clone for Binding<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.share(),
        }
    }
}

/// Event signal that can be triggered
/// 
/// Similar to Delphi's event handlers (OnClick, OnChange, etc.)
pub struct Event<T = ()> {
    handlers: Rc<RefCell<Vec<(usize, Box<dyn Fn(&T)>)>>>,
}

impl<T: 'static> Event<T> {
    /// Create a new event
    pub fn new() -> Self {
        Self {
            handlers: Rc::new(RefCell::new(Vec::new())),
        }
    }
    
    /// Trigger the event with a value
    pub fn emit(&self, value: &T) {
        for (_, handler) in self.handlers.borrow().iter() {
            handler(value);
        }
    }
    
    /// Subscribe to the event
    pub fn on(&self, handler: impl Fn(&T) + 'static) -> Subscription {
        let id = next_id();
        self.handlers.borrow_mut().push((id, Box::new(handler)));
        
        let handlers = Rc::clone(&self.handlers);
        Subscription::new(id, move || {
            handlers.borrow_mut().retain(|(h_id, _)| *h_id != id);
        })
    }
}

impl<T: 'static> Default for Event<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: 'static> Clone for Event<T> {
    fn clone(&self) -> Self {
        Self {
            handlers: Rc::clone(&self.handlers),
        }
    }
}

// =============================================================================
// SIGNAL SYSTEM - Simpler than Qt signals/slots, more ergonomic than SwiftUI
// =============================================================================

/// A Signal is a named event channel that can be connected to handlers.
/// 
/// Unlike Qt's signals/slots which require moc and complex macros,
/// GlassUI signals are just Rust structs with simple API.
///
/// # Example
/// ```rust
/// // Define signals in your widget
/// struct MyButton {
///     clicked: Signal<()>,
///     value_changed: Signal<i32>,
/// }
///
/// // Connect handlers
/// button.clicked.connect(|| println!("Clicked!"));
/// button.value_changed.connect(|v| println!("Value: {}", v));
///
/// // Emit signals
/// button.clicked.emit(());
/// button.value_changed.emit(42);
/// ```
pub struct Signal<T = ()> {
    handlers: Rc<RefCell<Vec<(usize, Box<dyn Fn(T)>)>>>,
}

impl<T: Clone + 'static> Signal<T> {
    /// Create a new signal
    pub fn new() -> Self {
        Self {
            handlers: Rc::new(RefCell::new(Vec::new())),
        }
    }
    
    /// Connect a handler to this signal
    /// Returns a Connection that auto-disconnects when dropped
    pub fn connect<F>(&self, handler: F) -> Connection 
    where 
        F: Fn(T) + 'static 
    {
        let id = next_id();
        self.handlers.borrow_mut().push((id, Box::new(handler)));
        
        let handlers = Rc::clone(&self.handlers);
        Connection::new(id, move || {
            handlers.borrow_mut().retain(|(h_id, _)| *h_id != id);
        })
    }
    
    /// Connect a handler and keep it alive forever
    pub fn connect_forever<F>(&self, handler: F) 
    where 
        F: Fn(T) + 'static 
    {
        self.connect(handler).forget();
    }
    
    /// Emit the signal, calling all connected handlers
    pub fn emit(&self, value: T) {
        for (_, handler) in self.handlers.borrow().iter() {
            handler(value.clone());
        }
    }
    
    /// Check if any handlers are connected
    pub fn is_connected(&self) -> bool {
        !self.handlers.borrow().is_empty()
    }
    
    /// Disconnect all handlers
    pub fn disconnect_all(&self) {
        self.handlers.borrow_mut().clear();
    }
}

impl<T: Clone + 'static> Default for Signal<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + 'static> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            handlers: Rc::clone(&self.handlers),
        }
    }
}

/// Simplified alias for Signal<()>
pub type Action = Signal<()>;

/// Connection handle - automatically disconnects when dropped
pub struct Connection {
    id: usize,
    disconnect: Option<Box<dyn FnOnce()>>,
}

impl Connection {
    pub fn new(id: usize, disconnect: impl FnOnce() + 'static) -> Self {
        Self {
            id,
            disconnect: Some(Box::new(disconnect)),
        }
    }
    
    /// Keep connection alive (prevent auto-disconnect)
    pub fn forget(mut self) {
        self.disconnect = None;
    }
    
    /// Manually disconnect
    pub fn disconnect(mut self) {
        if let Some(d) = self.disconnect.take() {
            d();
        }
    }
    
    pub fn id(&self) -> usize {
        self.id
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        if let Some(d) = self.disconnect.take() {
            d();
        }
    }
}

// =============================================================================
// EFFECT - Auto-tracking reactive effects (like React useEffect + Solid.js)
// =============================================================================

/// Holds multiple connections that are managed together
pub struct EffectScope {
    connections: Vec<Connection>,
    subscriptions: Vec<Subscription>,
}

impl EffectScope {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
            subscriptions: Vec::new(),
        }
    }
    
    /// Add a connection to this scope
    pub fn add_connection(&mut self, conn: Connection) {
        self.connections.push(conn);
    }
    
    /// Add a subscription to this scope
    pub fn add_subscription(&mut self, sub: Subscription) {
        self.subscriptions.push(sub);
    }
    
    /// Watch a state and run effect when it changes
    pub fn watch<T: Clone + 'static>(&mut self, state: &State<T>, effect: impl Fn(&T) + 'static) {
        let sub = state.subscribe_immediate(effect);
        self.subscriptions.push(sub);
    }
    
    /// Watch two states and run effect when either changes
    pub fn watch2<T1: Clone + 'static, T2: Clone + 'static>(
        &mut self, 
        state1: &State<T1>, 
        state2: &State<T2>, 
        effect: impl Fn(&T1, &T2) + Clone + 'static
    ) {
        let s2 = state2.share();
        let e1 = effect.clone();
        let sub1 = state1.subscribe(move |v1| {
            e1(v1, &s2.get());
        });
        
        let s1 = state1.share();
        let sub2 = state2.subscribe(move |v2| {
            effect(&s1.get(), v2);
        });
        
        self.subscriptions.push(sub1);
        self.subscriptions.push(sub2);
    }
    
    /// Clear all connections and subscriptions
    pub fn clear(&mut self) {
        self.connections.clear();
        self.subscriptions.clear();
    }
}

impl Default for EffectScope {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// ASYNC + REACTIVE - Seamless async data loading with reactive updates
// =============================================================================

/// Loading state for async operations
#[derive(Clone, Debug, PartialEq)]
pub enum LoadingState<T, E = String> {
    /// Initial state, not yet loaded
    Idle,
    /// Currently loading
    Loading,
    /// Successfully loaded with data
    Ready(T),
    /// Failed to load
    Error(E),
}

impl<T, E> LoadingState<T, E> {
    pub fn is_idle(&self) -> bool { matches!(self, LoadingState::Idle) }
    pub fn is_loading(&self) -> bool { matches!(self, LoadingState::Loading) }
    pub fn is_ready(&self) -> bool { matches!(self, LoadingState::Ready(_)) }
    pub fn is_error(&self) -> bool { matches!(self, LoadingState::Error(_)) }
    
    pub fn data(&self) -> Option<&T> {
        match self {
            LoadingState::Ready(data) => Some(data),
            _ => None,
        }
    }
    
    pub fn error(&self) -> Option<&E> {
        match self {
            LoadingState::Error(e) => Some(e),
            _ => None,
        }
    }
}

impl<T: Default, E> Default for LoadingState<T, E> {
    fn default() -> Self {
        LoadingState::Idle
    }
}

/// Async Resource - combines async data fetching with reactive state
/// 
/// Similar to React Query, SWR, or Solid's createResource.
/// Automatically tracks loading/error states.
///
/// # Example
/// ```rust
/// // Create a resource that fetches user data
/// let user = Resource::new(|| async {
///     fetch_user_from_api().await
/// });
///
/// // Trigger the fetch
/// user.load();
///
/// // React to state changes
/// match user.state().get() {
///     LoadingState::Loading => show_spinner(),
///     LoadingState::Ready(data) => show_user(data),
///     LoadingState::Error(e) => show_error(e),
///     _ => {}
/// }
/// ```
pub struct Resource<T: Clone + 'static> {
    state: State<LoadingState<T, String>>,
    refetch_signal: Signal<()>,
}

impl<T: Clone + 'static> Resource<T> {
    /// Create a new resource (starts in Idle state)
    pub fn new() -> Self {
        Self {
            state: State::new(LoadingState::Idle),
            refetch_signal: Signal::new(),
        }
    }
    
    /// Get the current loading state
    pub fn state(&self) -> &State<LoadingState<T, String>> {
        &self.state
    }
    
    /// Get the data if ready
    pub fn data(&self) -> Option<T> {
        match self.state.get() {
            LoadingState::Ready(data) => Some(data),
            _ => None,
        }
    }
    
    /// Check if currently loading
    pub fn is_loading(&self) -> bool {
        self.state.get().is_loading()
    }
    
    /// Set to loading state
    pub fn start_loading(&self) {
        self.state.set(LoadingState::Loading);
    }
    
    /// Set successful result
    pub fn set_data(&self, data: T) {
        self.state.set(LoadingState::Ready(data));
    }
    
    /// Set error state
    pub fn set_error(&self, error: impl Into<String>) {
        self.state.set(LoadingState::Error(error.into()));
    }
    
    /// Reset to idle state
    pub fn reset(&self) {
        self.state.set(LoadingState::Idle);
    }
    
    /// Get signal that fires when refetch is requested
    pub fn on_refetch(&self) -> &Signal<()> {
        &self.refetch_signal
    }
    
    /// Request a refetch (emits refetch signal)
    pub fn refetch(&self) {
        self.refetch_signal.emit(());
    }
    
    /// Subscribe to state changes
    pub fn subscribe(&self, f: impl Fn(&LoadingState<T, String>) + 'static) -> Subscription {
        self.state.subscribe(f)
    }
}

impl<T: Clone + 'static> Default for Resource<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + 'static> Clone for Resource<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            refetch_signal: self.refetch_signal.clone(),
        }
    }
}

/// Channel for sending values from async contexts to the UI thread
/// 
/// Used for async operations that need to update reactive state.
/// 
/// # Example
/// ```rust
/// let (sender, receiver) = channel::<String>();
/// 
/// // In async task
/// sender.send("Hello from async!".to_string());
/// 
/// // In UI, poll for messages
/// if let Some(msg) = receiver.try_recv() {
///     label_text.set(msg);
/// }
/// ```
pub struct Sender<T> {
    queue: Rc<RefCell<Vec<T>>>,
}

pub struct Receiver<T> {
    queue: Rc<RefCell<Vec<T>>>,
}

/// Create a channel for async -> UI communication
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let queue = Rc::new(RefCell::new(Vec::new()));
    (
        Sender { queue: Rc::clone(&queue) },
        Receiver { queue },
    )
}

impl<T> Sender<T> {
    /// Send a value (thread-safe for single-threaded async)
    pub fn send(&self, value: T) {
        self.queue.borrow_mut().push(value);
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self { queue: Rc::clone(&self.queue) }
    }
}

impl<T> Receiver<T> {
    /// Try to receive a value (non-blocking)
    pub fn try_recv(&self) -> Option<T> {
        let mut queue = self.queue.borrow_mut();
        if queue.is_empty() {
            None
        } else {
            Some(queue.remove(0))
        }
    }
    
    /// Receive all pending values
    pub fn recv_all(&self) -> Vec<T> {
        std::mem::take(&mut *self.queue.borrow_mut())
    }
    
    /// Check if there are pending messages
    pub fn has_pending(&self) -> bool {
        !self.queue.borrow().is_empty()
    }
}

/// Convenience macro for creating state
#[macro_export]
macro_rules! state {
    ($value:expr) => {
        $crate::state::State::new($value)
    };
}

/// Convenience macro for connecting signals (Qt-style but simpler)
/// 
/// # Example
/// ```rust
/// connect!(button.clicked => || save_data());
/// connect!(slider.value_changed => |v| update_display(v));
/// connect!(checkbox.toggled => self, on_toggle);  // Method syntax
/// ```
#[macro_export]
macro_rules! connect {
    // Signal to closure
    ($signal:expr => $handler:expr) => {
        $signal.connect($handler)
    };
    
    // Signal to method
    ($signal:expr => $self:ident, $method:ident) => {
        $signal.connect(move |v| $self.$method(v))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    
    #[test]
    fn test_state_get_set() {
        let state = State::new(0);
        assert_eq!(state.get(), 0);
        state.set(42);
        assert_eq!(state.get(), 42);
    }
    
    #[test]
    fn test_state_subscribe() {
        let state = State::new(0);
        let called = Rc::new(Cell::new(0));
        let called_clone = Rc::clone(&called);
        
        let _sub = state.subscribe(move |v| {
            called_clone.set(*v);
        });
        
        state.set(5);
        assert_eq!(called.get(), 5);
        
        state.set(10);
        assert_eq!(called.get(), 10);
    }
    
    #[test]
    fn test_subscription_drop() {
        let state = State::new(0);
        let called = Rc::new(Cell::new(0));
        let called_clone = Rc::clone(&called);
        
        {
            let _sub = state.subscribe(move |v| {
                called_clone.set(*v);
            });
            state.set(5);
            assert_eq!(called.get(), 5);
        }
        
        // Subscription dropped, should not be called
        state.set(100);
        assert_eq!(called.get(), 5); // Still 5, not 100
    }
    
    #[test]
    fn test_computed() {
        let count = State::new(5);
        let doubled = Computed::from(&count, |v| v * 2);
        
        assert_eq!(doubled.get(), 10);
        
        count.set(7);
        assert_eq!(doubled.get(), 14);
    }
    
    #[test]
    fn test_event() {
        let event: Event<i32> = Event::new();
        let received = Rc::new(Cell::new(0));
        let received_clone = Rc::clone(&received);
        
        let _sub = event.on(move |v| {
            received_clone.set(*v);
        });
        
        event.emit(&42);
        assert_eq!(received.get(), 42);
    }
}
