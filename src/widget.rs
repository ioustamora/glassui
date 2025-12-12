use glam::{Vec2, Vec4};
use winit::event::{ElementState, MouseButton};
use crate::renderer::GlassRenderer;

pub trait Widget {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2;
    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool;
    fn update(&mut self, dt: f32);
    fn render(&self, renderer: &mut GlassRenderer);
}

// --- Layouts ---

pub struct Column {
    pub position: Vec2,
    pub size: Vec2,
    pub children: Vec<Box<dyn Widget>>,
    pub spacing: f32,
    pub padding: f32,
}

impl Column {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            children: Vec::new(),
            spacing: 10.0,
            padding: 10.0,
        }
    }
    
    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }
}

impl Widget for Column {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        
        let mut cursor = origin + Vec2::splat(self.padding);
        let content_width = max_size.x - self.padding * 2.0;
        let mut max_width = 0.0f32;
        
        for child in &mut self.children {
            let used_height = cursor.y - origin.y - self.padding;
            let remaining_height = (max_size.y - self.padding * 2.0 - used_height).max(0.0);
            
            let child_size = child.layout(cursor, Vec2::new(content_width, remaining_height)); 
            cursor.y += child_size.y + self.spacing;
            max_width = max_width.max(child_size.x);
        }
        
        self.size = Vec2::new(max_width + self.padding * 2.0, cursor.y - origin.y + self.padding - self.spacing); // Remove last spacing
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let mut handled = false;
        for child in &mut self.children {
            if child.handle_event(event, mouse_pos) {
                handled = true;
            }
        }
        handled
    }

    fn update(&mut self, dt: f32) {
        for child in &mut self.children {
            child.update(dt);
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        // Transparent debug bg?
        // renderer.draw_rect(self.position, self.size, Vec4::new(1.0, 0.0, 0.0, 0.1));
        for child in &self.children {
            child.render(renderer);
        }
    }
}

pub struct Row {
    pub position: Vec2,
    pub size: Vec2,
    pub children: Vec<Box<dyn Widget>>,
    pub spacing: f32,
    pub padding: f32,
}

impl Row {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            children: Vec::new(),
            spacing: 10.0,
            padding: 10.0,
        }
    }
    
    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }
}

impl Widget for Row {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        
        let mut cursor = origin + Vec2::splat(self.padding);
        let mut max_height = 0.0f32;
        
        for child in &mut self.children {
            let used_width = cursor.x - origin.x - self.padding;
            let remaining_width = (max_size.x - self.padding * 2.0 - used_width).max(0.0);
            
            let child_size = child.layout(cursor, Vec2::new(remaining_width, max_size.y)); 
            cursor.x += child_size.x + self.spacing;
            max_height = max_height.max(child_size.y);
        }
        
        self.size = Vec2::new(cursor.x - origin.x + self.padding - self.spacing, max_height + self.padding * 2.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let mut handled = false;
        for child in &mut self.children {
            if child.handle_event(event, mouse_pos) {
                handled = true;
            }
        }
        handled
    }

    fn update(&mut self, dt: f32) {
        for child in &mut self.children {
            child.update(dt);
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        for child in &self.children {
            child.render(renderer);
        }
    }
}

pub struct Panel {
    pub position: Vec2,
    pub size: Vec2,
    pub content: Option<Box<dyn Widget>>,
    pub color: Vec4,
    pub fill: bool,
}

impl Panel {
    pub fn new(content: Box<dyn Widget>) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            content: Some(content),
            color: Vec4::new(1.0, 1.0, 1.0, 0.05),
            fill: false,
        }
    }
    
    pub fn new_empty() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            content: None,
            color: Vec4::new(1.0, 1.0, 1.0, 0.05),
            fill: false,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
    
    pub fn with_fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }
}

impl Widget for Panel {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        
        let padding = 20.0;
        
        // IMPORTANT: We MUST propagate the full `max_size` (minus padding) to the child content.
        // If Resizable is wrapping us, `max_size` IS the resized size.
        // We tell content: "You have this much text area, do what you can".
        
        let content_available = max_size - Vec2::splat(padding * 2.0);

        let content_size = if let Some(content) = &mut self.content {
            let content_origin = origin + Vec2::splat(padding);
            content.layout(content_origin, content_available)
        } else {
            Vec2::ZERO
        };
        
        if self.fill {
            self.size = max_size;
        } else {
            self.size = content_size + Vec2::splat(padding * 2.0);
        }
        
        self.size 
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        if let Some(content) = &mut self.content {
            return content.handle_event(event, mouse_pos);
        }
        false
    }

    fn update(&mut self, dt: f32) {
        if let Some(content) = &mut self.content {
            content.update(dt);
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        renderer.draw_rect(self.position, self.size, self.color);
        if let Some(content) = &self.content {
            content.render(renderer);
        }
    }
}

// --- Start Basic Widgets (Button, etc...) ---
pub struct Button {
    pub position: Vec2,
    pub size: Vec2,
    pub text: String,
    pub hovered: bool,
    pub pressed: bool,
    pub hover_t: f32,
    pub press_t: f32,
    pub on_click: Option<Box<dyn FnMut()>>,
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::splat(0.0),
            text: text.to_string(),
            hovered: false,
            pressed: false,
            hover_t: 0.0,
            press_t: 0.0,
            on_click: None,
        }
    }
}

impl Widget for Button {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(200.0, 50.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let rect_min = self.position;
        let rect_max = self.position + self.size;
        let inside = mouse_pos.x >= rect_min.x && mouse_pos.x <= rect_max.x &&
                     mouse_pos.y >= rect_min.y && mouse_pos.y <= rect_max.y;
        
        self.hovered = inside;
        
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state, button: MouseButton::Left, .. }, .. } = event {
            if *state == ElementState::Pressed {
                if inside {
                    self.pressed = true;
                    return true;
                }
            } else if *state == ElementState::Released {
                if self.pressed && inside {
                    println!("Button '{}' clicked!", self.text);
                    if let Some(callback) = &mut self.on_click {
                        callback();
                    }
                    self.pressed = false;
                    return true;
                }
                self.pressed = false;
            }
        }
        false
    }

    fn update(&mut self, dt: f32) {
        let hover_target = if self.hovered { 1.0 } else { 0.0 };
        self.hover_t += (hover_target - self.hover_t) * 15.0 * dt;
        
        let press_target = if self.pressed { 1.0 } else { 0.0 };
        self.press_t += (press_target - self.press_t) * 20.0 * dt;
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let base_col = Vec4::new(0.0, 0.0, 0.0, 0.3);
        let hover_col = Vec4::new(0.0, 0.1, 0.2, 0.5);
        let press_col = Vec4::new(0.0, 0.3, 0.4, 0.8);
        
        let mut color = base_col.lerp(hover_col, self.hover_t);
        color = color.lerp(press_col, self.press_t);
        
        let scale = self.hover_t * 4.0 - self.press_t * 2.0; // Expand on hover, shrink on press
        
        if self.hover_t > 0.01 {
             renderer.draw_rect(
                self.position - Vec2::splat(scale),
                self.size + Vec2::splat(scale*2.0),
                Vec4::new(0.0, 0.8, 1.0, (self.hover_t * 0.3).max(self.press_t * 0.5))
             );
        }
        // Draw text centered-ish
        let text_len = self.text.len() as f32 * 10.0; 
        let text_pos = self.position + (self.size - Vec2::new(text_len, 20.0)) * 0.5 + Vec2::new(0.0, self.press_t * 2.0); // Offset down on press

        renderer.draw_rect(self.position + Vec2::splat(self.press_t * 2.0), self.size - Vec2::splat(self.press_t * 4.0), color + Vec4::new(0.0,0.0,0.0,0.2));
        renderer.draw_text(&self.text, text_pos, 20.0, Vec4::new(1.0, 1.0, 1.0, 1.0));
    }
}

pub struct Label {
    pub position: Vec2,
    pub text: String,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self { position: Vec2::ZERO, text: text.to_string() }
    }
}

impl Widget for Label {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        Vec2::new(100.0, 20.0) 
    }
    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool { false }
    fn update(&mut self, _dt: f32) {}
    fn render(&self, renderer: &mut GlassRenderer) {
        renderer.draw_text(&self.text, self.position, 24.0, Vec4::new(1.0, 1.0, 1.0, 0.9));
    }
}

pub struct Slider {
    pub position: Vec2,
    pub size: Vec2,
    pub value: f32, // 0.0 to 1.0
    pub dragging: bool,
}

impl Slider {
    pub fn new(value: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            value: value.clamp(0.0, 1.0),
            dragging: false,
        }
    }
}

impl Widget for Slider {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(200.0, 20.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let rect_min = self.position;
        let rect_max = self.position + self.size;
        let inside = mouse_pos.x >= rect_min.x && mouse_pos.x <= rect_max.x &&
                     mouse_pos.y >= rect_min.y && mouse_pos.y <= rect_max.y;

        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state, button: MouseButton::Left, .. }, .. } => {
                if *state == ElementState::Pressed && inside {
                    self.dragging = true;
                } else if *state == ElementState::Released {
                    self.dragging = false;
                }
            }
            _ => { 
                 if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. }, .. } = event {
                    self.dragging = false;
                }
            }
        }
        
        if self.dragging {
            let relative_x = (mouse_pos.x - self.position.x).clamp(0.0, self.size.x);
            self.value = relative_x / self.size.x;
            return true;
        }
        
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        // Track
        renderer.draw_rect(self.position, self.size, Vec4::new(0.0, 0.0, 0.0, 0.5));
        
        // Fill
        let fill_width = self.size.x * self.value;
        renderer.draw_rect(self.position, Vec2::new(fill_width, self.size.y), Vec4::new(0.0, 1.0, 1.0, 0.5));
        
        // Handle
        let handle_pos = Vec2::new(self.position.x + fill_width - 5.0, self.position.y - 2.0);
        renderer.draw_rect(handle_pos, Vec2::new(10.0, self.size.y + 4.0), Vec4::new(1.0, 1.0, 1.0, 0.9));
    }
}

pub struct Checkbox {
    pub position: Vec2,
    pub size: Vec2,
    pub checked: bool,
    pub label: String,
}

impl Checkbox {
    pub fn new(label: &str, checked: bool) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            checked,
            label: label.to_string(),
        }
    }
}

impl Widget for Checkbox {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(200.0, 20.0);
        Vec2::new(200.0, 20.0)
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let rect_min = self.position;
        let rect_max = self.position + self.size;
        let inside = mouse_pos.x >= rect_min.x && mouse_pos.x <= rect_max.x &&
                     mouse_pos.y >= rect_min.y && mouse_pos.y <= rect_max.y;
                     
        if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } = event {
            if inside {
                self.checked = !self.checked;
                return true;
            }
        }
        false
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
         let color = if self.checked { Vec4::new(0.0, 1.0, 0.0, 0.6) } else { Vec4::new(0.2, 0.2, 0.2, 0.5) };
        renderer.draw_rect(self.position, self.size, color);
        
        if self.checked {
            renderer.draw_rect(self.position - Vec2::splat(2.0), self.size + Vec2::splat(4.0), Vec4::new(0.0, 1.0, 0.0, 0.3));
        }
        
        // Label for checkbox
        renderer.draw_text(&self.label, self.position + Vec2::new(30.0, 0.0), 16.0, Vec4::new(1.0, 1.0, 1.0, 0.8));
    }
}

// --- Helper Widgets ---

pub struct Spacer {
    pub size: Vec2,
}

impl Spacer {
    pub fn new(size: Vec2) -> Self {
        Self { size }
    }
}

impl Widget for Spacer {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        // Spacer takes requested size, ignores limits? Or clamps?
        // Let's just return size.
        self.size
    }
    fn handle_event(&mut self, _event: &winit::event::Event<()>, _mouse_pos: Vec2) -> bool { false }
    fn update(&mut self, _dt: f32) {}
    fn render(&self, _renderer: &mut GlassRenderer) {}
}

pub enum Alignment {
    Center,
    TopLeft,
    BottomLeft,
}

pub struct Align {
    pub alignment: Alignment,
    pub child: Box<dyn Widget>,
    // We need position/size for the wrapper itself to render/debug or just pass through?
    // The wrapper fills the max_size offered to it, and places child inside.
    pub position: Vec2,
    pub size: Vec2,
}

impl Align {
    pub fn new(alignment: Alignment, child: Box<dyn Widget>) -> Self {
        Self {
            alignment,
            child,
            position: Vec2::ZERO,
            size: Vec2::ZERO,
        }
    }
}

impl Widget for Align {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = max_size; // Fill available space
        
        // 1. Probe size: (hack: layout far away?) -> actually layout returns size.
        // Let's assume layout sets position, but we can move it? 
        // Widget trait doesn't have "set_position". layout *does* it. It's imperative.
        
        // Strategy: 
        // 1. Layout child at origin (0,0) with max_size.
        // 2. Get child size.
        // 3. Calculate aligned position.
        // 4. Layout child *again* at aligned position.
        
        // Is calling layout twice safe? Yes, mostly just sets state.
        
        let child_size = self.child.layout(origin, max_size);
        
        let mut final_pos = origin;
        match self.alignment {
            Alignment::Center => {
                final_pos = origin + (max_size - child_size) * 0.5;
            },
            Alignment::TopLeft => {
                final_pos = origin;
            },
            Alignment::BottomLeft => {
                final_pos = Vec2::new(origin.x, origin.y + max_size.y - child_size.y);
            }
        }
        
        self.child.layout(final_pos, max_size);
        
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        self.child.handle_event(event, mouse_pos)
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        self.child.render(renderer);
    }
}

pub struct Stack {
    pub position: Vec2,
    pub size: Vec2,
    pub children: Vec<Box<dyn Widget>>,
}

impl Stack {
    pub fn new() -> Self {
        Self { position: Vec2::ZERO, size: Vec2::ZERO, children: Vec::new() }
    }
    
    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }
}

impl Widget for Stack {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = max_size;
        
        for child in &mut self.children {
            child.layout(origin, max_size);
        }
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        // Reverse order for events (top-most first)
        let mut handled_index = None;
        let len = self.children.len();
        
        for i in (0..len).rev() {
            if self.children[i].handle_event(event, mouse_pos) {
                handled_index = Some(i);
                break;
            }
        }
        
        if let Some(index) = handled_index {
            // Bring to front (end of list) if interaction happened
            // Only strictly necessary if not already at end, but cheap check
            if index != len - 1 {
                let child = self.children.remove(index);
                self.children.push(child);
            }
            return true;
        }
        
        false
    }
    
    fn update(&mut self, dt: f32) {
        for child in &mut self.children {
            child.update(dt);
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        for child in &self.children {
            child.render(renderer);
        }
    }
}

pub struct Draggable {
    pub position: Vec2, // Current global offset
    pub size: Vec2,     // Tracked size
    pub child: Box<dyn Widget>,
    pub dragging: bool,
    pub drag_start_mouse: Vec2,
    pub drag_start_pos: Vec2,
}

impl Draggable {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            child,
            dragging: false,
            drag_start_mouse: Vec2::ZERO,
            drag_start_pos: Vec2::ZERO,
        }
    }
}

impl Widget for Draggable {
    // layout receives the "intended" origin from parent (e.g. Align center)
    // but Draggable overrides it with its own offset?
    // OR, Draggable *modifies* the offset?
    // Let's say Draggable starts at 0,0 relative to parent.
    // If we drag it to 100,100, we want it to stay there.
    
    // Problem: Layout is stateless/immediate in parent. Parent says "Go to X".
    // If Draggable honors X, it snaps back.
    // If Draggable ignores X and uses internal State, it works.
    
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        // We only initialize position once? Or tracking delta?
        // Let's track absolute position.
        // If position is ZERO (fresh), take origin.
        if self.position == Vec2::ZERO && !self.dragging {
             // Heuristic: If we haven't dragged yet, accept parent's suggestion.
             // But valid position could be 0,0.
             // We need a "initialized" flag or just rely on constructor?
             // Let's just Add our offset to origin?
             // No, "Draggable" usually implies Free Positioning.
             // The parent (Stack) gives us full screen (Align TopLeft).
             
             // If we are inside Align::Center, origin is Center. 
             // We want to be at Center + Offset.
             self.position = origin;
        }
        
        // However, if we are dragging, we are updating self.position manually.
        // The parent might keep sending us same origin.
        
        // Wait, if we use Align::Center, origin changes on resize.
        // We want stable relative to window? Or absolute?
        // Let's go with: Draggable maintains an OFFSET from the layout origin.
        
        // Actually, simple "Window" behavior is usually absolute.
        // But for this layout system, let's treat `self.position` as the final screen coordinate we want.
        // But we have to return size.
        
        let child_size = self.child.layout(self.position, max_size);
        self.size = child_size;
        
        // If we are NOT dragging, and resize happens... 
        // We might want to respect flow? No. Draggable breaks flow.
        
        child_size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        // 1. Child gets first dibs (buttons inside window)
        if self.child.handle_event(event, mouse_pos) {
            return true;
        }
        
        // 2. Drag logic
        let rect_min = self.position;
        let rect_max = self.position + self.size;
        let inside = mouse_pos.x >= rect_min.x && mouse_pos.x <= rect_max.x &&
                     mouse_pos.y >= rect_min.y && mouse_pos.y <= rect_max.y;
                     
        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state, button: MouseButton::Left, .. }, .. } => {
                if *state == ElementState::Pressed && inside {
                    self.dragging = true;
                    self.drag_start_mouse = mouse_pos;
                    self.drag_start_pos = self.position;
                    // println!("Drag Started at {:?}", self.position);
                    return true;
                } else if *state == ElementState::Released {
                     if self.dragging {
                        // println!("Drag Ended at {:?}", self.position);
                     }
                    self.dragging = false;
                }
            },
            _ => {
                 if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. }, .. } = event {
                    self.dragging = false;
                }
            }
        }
        
        if self.dragging {
            let delta = mouse_pos - self.drag_start_mouse;
            self.position = self.drag_start_pos + delta;
            return true;
        }

        false
    }
    
    fn update(&mut self, dt: f32) {
        self.child.update(dt);
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        self.child.render(renderer);
    }
}

pub struct Resizable {
    pub position: Vec2,
    pub current_size: Vec2,
    pub min_size: Vec2,
    pub child: Box<dyn Widget>,
    pub resizing: bool,
    pub resize_start_mouse: Vec2,
    pub resize_start_size: Vec2,
}

impl Resizable {
    pub fn new(child: Box<dyn Widget>, initial_size: Vec2) -> Self {
        Self {
            position: Vec2::ZERO,
            current_size: initial_size,
            min_size: Vec2::new(100.0, 100.0),
            child,
            resizing: false,
            resize_start_mouse: Vec2::ZERO,
            resize_start_size: Vec2::ZERO,
        }
    }
}

impl Widget for Resizable {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        // Enforce our current_size on the child
        // We act as a constraint box.
        self.child.layout(origin, self.current_size);
        self.current_size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        // 1. Resize Handle Logic
        // Handle area: bottom-right 20x20
        let handle_rect = self.position + self.current_size - Vec2::splat(20.0);
        let in_handle = mouse_pos.x >= handle_rect.x && mouse_pos.y >= handle_rect.y &&
                        mouse_pos.x <= self.position.x + self.current_size.x &&
                        mouse_pos.y <= self.position.y + self.current_size.y;

        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state, button: MouseButton::Left, .. }, .. } => {
                if *state == ElementState::Pressed && in_handle {
                    self.resizing = true;
                    self.resize_start_mouse = mouse_pos;
                    self.resize_start_size = self.current_size;
                    return true;
                } else if *state == ElementState::Released {
                    self.resizing = false;
                }
            },
             _ => {
                 if let winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. }, .. } = event {
                    self.resizing = false;
                }
            }
        }

        if self.resizing {
            let delta = mouse_pos - self.resize_start_mouse;
            self.current_size = (self.resize_start_size + delta).max(self.min_size);
            return true;
        }

        // 2. Child events
        if self.child.handle_event(event, mouse_pos) {
            return true;
        }
        
        false
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        self.child.render(renderer);
        
        // Draw resize handle (simple triangle or rect in corner)
        let handle_pos = self.position + self.current_size - Vec2::splat(15.0);
        renderer.draw_rect(handle_pos, Vec2::splat(10.0), Vec4::new(1.0, 1.0, 1.0, 0.3));
    }
}

pub struct TextInput {
    pub position: Vec2,
    pub size: Vec2,
    pub text: String,
    pub focused: bool,
    pub cursor_visible: bool,
    pub cursor_timer: f32,
}

impl TextInput {
    pub fn new(placeholder: &str) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            text: placeholder.to_string(),
            focused: false,
            cursor_visible: true,
            cursor_timer: 0.0,
        }
    }
}

impl Widget for TextInput {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(200.0, 30.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let rect_min = self.position;
        let rect_max = self.position + self.size;
        let inside = mouse_pos.x >= rect_min.x && mouse_pos.x <= rect_max.x &&
                     mouse_pos.y >= rect_min.y && mouse_pos.y <= rect_max.y;

        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } => {
                if inside {
                    self.focused = true;
                    // If clicking inside, consume event
                    return true;
                } else {
                    self.focused = false;
                }
            },
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::KeyboardInput { event: key_event, .. }, .. } => {
                if self.focused && key_event.state.is_pressed() {
                   if let winit::keyboard::Key::Named(winit::keyboard::NamedKey::Backspace) = key_event.logical_key {
                        self.text.pop();
                        return true;
                   }
                   
                   if let Some(text) = &key_event.text {
                       // Filter control characters if any (simple check)
                       if let Some(c) = text.chars().next() {
                           if !c.is_control() {
                               self.text.push_str(text);
                               return true;
                           }
                       }
                   }
                }
            },
            _ => {}
        }
        
        false
    }

    fn update(&mut self, dt: f32) {
        if self.focused {
            self.cursor_timer += dt;
            if self.cursor_timer > 0.5 {
                self.cursor_visible = !self.cursor_visible;
                self.cursor_timer = 0.0;
            }
        } else {
            self.cursor_visible = false;
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        // Background
        let bg_col = if self.focused { Vec4::new(0.1, 0.1, 0.1, 0.8) } else { Vec4::new(0.0, 0.0, 0.0, 0.5) };
        renderer.draw_rect(self.position, self.size, bg_col);
        
        // Border
        if self.focused {
            renderer.draw_rect(self.position - Vec2::splat(1.0), self.size + Vec2::splat(2.0), Vec4::new(0.0, 0.8, 1.0, 0.5));
        }

        // Text
        renderer.draw_text(&self.text, self.position + Vec2::new(5.0, 5.0), 18.0, Vec4::ONE);
        
        // Cursor
        if self.focused && self.cursor_visible {
            let text_width = self.text.len() as f32 * 9.0; // Rough calc
            let cursor_pos = self.position + Vec2::new(5.0 + text_width, 5.0);
            renderer.draw_rect(cursor_pos, Vec2::new(2.0, 20.0), Vec4::ONE);
        }
    }
}

pub struct ScrollArea {
    pub position: Vec2,
    pub size: Vec2,
    pub child: Box<dyn Widget>,
    pub scroll_offset: f32,
    pub content_height: f32,
}

impl ScrollArea {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            child,
            scroll_offset: 0.0,
            content_height: 0.0,
        }
    }
}

impl Widget for ScrollArea {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = max_size;
        
        // Layout child with infinite height to get real size
        // Position child offset by scroll
        let child_origin = origin + Vec2::new(0.0, -self.scroll_offset);
        let child_size = self.child.layout(child_origin, Vec2::new(max_size.x, 10000.0));
        self.content_height = child_size.y;
        
        // Clamp scroll
        let max_scroll = (self.content_height - self.size.y).max(0.0);
        self.scroll_offset = self.scroll_offset.clamp(0.0, max_scroll);
        
        // Re-layout if clamped (optimization: only if changed significantly?)
        // self.child.layout(origin + Vec2::new(0.0, -self.scroll_offset), Vec2::new(max_size.x, 10000.0));
        
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let rect_min = self.position;
        let rect_max = self.position + self.size;
        let inside = mouse_pos.x >= rect_min.x && mouse_pos.x <= rect_max.x &&
                     mouse_pos.y >= rect_min.y && mouse_pos.y <= rect_max.y;

        // 1. Pass to child first
        if inside {
             // We need to pass event to child even if it thinks it's outside our clip?
             // Child checks against its own position (which is scrolled).
             // If child is at -500, and mouse is at 100, child is verified.
             
             // BUT: We should NOT interact with child if mouse is outside ScrollArea.
             // (We checked `inside` above).
             
             if self.child.handle_event(event, mouse_pos) {
                 return true;
             }
        }
        
        // 2. Scroll logic
        if inside {
             match event {
                 winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseWheel { delta, .. }, .. } => {
                     let scroll_amount = match delta {
                         winit::event::MouseScrollDelta::LineDelta(_, y) => y * 30.0,
                         winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                     };
                     
                     let old_scroll = self.scroll_offset;
                     let max_scroll = (self.content_height - self.size.y).max(0.0);
                     self.scroll_offset = (self.scroll_offset - scroll_amount).clamp(0.0, max_scroll);
                     
                     if (self.scroll_offset - old_scroll).abs() > 0.1 {
                         return true;
                     }
                 },
                 _ => {}
             }
        }
        
        false
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        // Set Scissor
        let x = self.position.x as u32;
        let y = self.position.y as u32;
        let w = self.size.x as u32;
        let h = self.size.y as u32;
        
        if w > 0 && h > 0 {
            renderer.set_scissor([x, y, w, h]);
            
            // Render Child
            self.child.render(renderer);
            
            renderer.clear_scissor();
            
            // Draw Scrollbar if needed
            if self.content_height > self.size.y {
                let ratio = self.size.y / self.content_height;
                let bar_h = self.size.y * ratio;
                let bar_y = self.position.y + (self.scroll_offset / self.content_height) * self.size.y;
                
                renderer.draw_rect(
                    Vec2::new(self.position.x + self.size.x - 6.0, bar_y),
                    Vec2::new(4.0, bar_h),
                    Vec4::new(1.0, 1.0, 1.0, 0.3)
                );
            }
        }
    }
}

pub struct Tooltip {
    pub position: Vec2,
    pub size: Vec2,
    pub child: Box<dyn Widget>,
    pub text: String,
    pub hovered: bool,
    pub hover_time: f32,
    pub mouse_pos: Vec2,
}

impl Tooltip {
    pub fn new(child: Box<dyn Widget>, text: &str) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            child,
            text: text.to_string(),
            hovered: false,
            hover_time: 0.0,
            mouse_pos: Vec2::ZERO,
        }
    }
}

impl Widget for Tooltip {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = self.child.layout(origin, max_size);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        self.mouse_pos = mouse_pos;
        let handled = self.child.handle_event(event, mouse_pos);
        
        // Check hover
        let rect_min = self.position;
        let rect_max = self.position + self.size;
        let inside = mouse_pos.x >= rect_min.x && mouse_pos.x <= rect_max.x &&
                     mouse_pos.y >= rect_min.y && mouse_pos.y <= rect_max.y;
        
        self.hovered = inside;
        
        handled
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
        if self.hovered {
            self.hover_time += dt;
        } else {
            self.hover_time = 0.0;
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        self.child.render(renderer);
        
        if self.hovered && self.hover_time > 0.5 {
            renderer.draw_tooltip(&self.text, self.mouse_pos + Vec2::new(10.0, 10.0));
        }
    }
}

// --- Context Menu ---

pub struct MenuItem {
    pub label: String,
    pub on_click: Option<Box<dyn FnMut()>>,
}

impl MenuItem {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            on_click: None,
        }
    }
    
    pub fn with_callback<F: FnMut() + 'static>(mut self, callback: F) -> Self {
        self.on_click = Some(Box::new(callback));
        self
    }
}

pub struct ContextMenu {
    pub position: Vec2,
    pub items: Vec<MenuItem>,
    pub visible: bool,
    pub item_height: f32,
    pub width: f32,
    pub hovered_index: Option<usize>,
}

impl ContextMenu {
    pub fn new(items: Vec<MenuItem>) -> Self {
        Self {
            position: Vec2::ZERO,
            items,
            visible: false,
            item_height: 30.0,
            width: 150.0,
            hovered_index: None,
        }
    }
    
    pub fn show(&mut self, pos: Vec2) {
        self.position = pos;
        self.visible = true;
    }
    
    pub fn hide(&mut self) {
        self.visible = false;
    }
    
    fn total_height(&self) -> f32 {
        self.items.len() as f32 * self.item_height
    }
}

impl Widget for ContextMenu {
    fn layout(&mut self, _origin: Vec2, _max_size: Vec2) -> Vec2 {
        // Context menu uses its own position (set by `show`)
        Vec2::new(self.width, self.total_height())
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }
        
        let rect_min = self.position;
        let rect_max = self.position + Vec2::new(self.width, self.total_height());
        let inside = mouse_pos.x >= rect_min.x && mouse_pos.x <= rect_max.x &&
                     mouse_pos.y >= rect_min.y && mouse_pos.y <= rect_max.y;
        
        // Calculate hovered item
        if inside {
            let relative_y = mouse_pos.y - self.position.y;
            let index = (relative_y / self.item_height) as usize;
            if index < self.items.len() {
                self.hovered_index = Some(index);
            } else {
                self.hovered_index = None;
            }
        } else {
            self.hovered_index = None;
        }
        
        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state, button, .. }, .. } => {
                if *state == ElementState::Pressed {
                    if *button == MouseButton::Left {
                        if inside {
                            // Click on item
                            if let Some(index) = self.hovered_index {
                                if let Some(callback) = &mut self.items[index].on_click {
                                    callback();
                                }
                                println!("Context menu item '{}' clicked!", self.items[index].label);
                                self.hide();
                                return true;
                            }
                        } else {
                            // Click outside - dismiss
                            self.hide();
                            return true;
                        }
                    }
                }
            },
            _ => {}
        }
        
        inside // Consume events if mouse is inside menu
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        if !self.visible {
            return;
        }
        
        // Background
        renderer.draw_rect(self.position, Vec2::new(self.width, self.total_height()), Vec4::new(0.1, 0.1, 0.15, 0.95));
        
        // Border
        renderer.draw_rect(self.position - Vec2::splat(1.0), Vec2::new(self.width + 2.0, self.total_height() + 2.0), Vec4::new(0.3, 0.3, 0.4, 0.5));
        
        // Items
        for (i, item) in self.items.iter().enumerate() {
            let item_pos = self.position + Vec2::new(0.0, i as f32 * self.item_height);
            
            // Hover highlight
            if self.hovered_index == Some(i) {
                renderer.draw_rect(item_pos, Vec2::new(self.width, self.item_height), Vec4::new(0.2, 0.4, 0.6, 0.8));
            }
            
            // Text
            renderer.draw_text(&item.label, item_pos + Vec2::new(10.0, 5.0), 16.0, Vec4::ONE);
        }
    }
}

pub struct ContextMenuTrigger {
    pub position: Vec2,
    pub size: Vec2,
    pub child: Box<dyn Widget>,
    pub menu: ContextMenu,
}

impl ContextMenuTrigger {
    pub fn new(child: Box<dyn Widget>, items: Vec<MenuItem>) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            child,
            menu: ContextMenu::new(items),
        }
    }
}

impl Widget for ContextMenuTrigger {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = self.child.layout(origin, max_size);
        self.menu.layout(Vec2::ZERO, max_size); // Menu positions itself
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        // Menu gets priority if visible
        if self.menu.visible {
            if self.menu.handle_event(event, mouse_pos) {
                return true;
            }
        }
        
        // Child events
        if self.child.handle_event(event, mouse_pos) {
            return true;
        }
        
        // Right-click detection
        let rect_min = self.position;
        let rect_max = self.position + self.size;
        let inside = mouse_pos.x >= rect_min.x && mouse_pos.x <= rect_max.x &&
                     mouse_pos.y >= rect_min.y && mouse_pos.y <= rect_max.y;
        
        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Right, .. }, .. } => {
                if inside {
                    self.menu.show(mouse_pos);
                    return true;
                }
            },
            _ => {}
        }
        
        false
    }

    fn update(&mut self, dt: f32) {
        self.child.update(dt);
        self.menu.update(dt);
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        self.child.render(renderer);
        self.menu.render(renderer);
    }
}

// --- Dropdown ---

pub struct Dropdown {
    pub position: Vec2,
    pub size: Vec2,
    pub options: Vec<String>,
    pub selected_index: usize,
    pub expanded: bool,
    pub hovered_index: Option<usize>,
    pub item_height: f32,
}

impl Dropdown {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            options,
            selected_index: 0,
            expanded: false,
            hovered_index: None,
            item_height: 30.0,
        }
    }
    
    pub fn with_selected(mut self, index: usize) -> Self {
        if index < self.options.len() {
            self.selected_index = index;
        }
        self
    }
    
    fn dropdown_height(&self) -> f32 {
        self.options.len() as f32 * self.item_height
    }
}

impl Widget for Dropdown {
    fn layout(&mut self, origin: Vec2, _max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = Vec2::new(200.0, 30.0);
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        let header_min = self.position;
        let header_max = self.position + self.size;
        let in_header = mouse_pos.x >= header_min.x && mouse_pos.x <= header_max.x &&
                        mouse_pos.y >= header_min.y && mouse_pos.y <= header_max.y;
        
        // Dropdown area
        let dropdown_min = self.position + Vec2::new(0.0, self.size.y);
        let dropdown_max = dropdown_min + Vec2::new(self.size.x, self.dropdown_height());
        let in_dropdown = self.expanded &&
                          mouse_pos.x >= dropdown_min.x && mouse_pos.x <= dropdown_max.x &&
                          mouse_pos.y >= dropdown_min.y && mouse_pos.y <= dropdown_max.y;
        
        // Calculate hovered option
        if in_dropdown {
            let relative_y = mouse_pos.y - dropdown_min.y;
            let index = (relative_y / self.item_height) as usize;
            if index < self.options.len() {
                self.hovered_index = Some(index);
            } else {
                self.hovered_index = None;
            }
        } else {
            self.hovered_index = None;
        }
        
        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } => {
                if in_header {
                    self.expanded = !self.expanded;
                    return true;
                } else if in_dropdown {
                    if let Some(index) = self.hovered_index {
                        self.selected_index = index;
                        println!("Dropdown selected: {}", self.options[index]);
                    }
                    self.expanded = false;
                    return true;
                } else if self.expanded {
                    // Click outside - close
                    self.expanded = false;
                    return true;
                }
            },
            _ => {}
        }
        
        in_header || in_dropdown
    }

    fn update(&mut self, _dt: f32) {}

    fn render(&self, renderer: &mut GlassRenderer) {
        // Header
        let header_color = if self.expanded {
            Vec4::new(0.15, 0.15, 0.2, 0.9)
        } else {
            Vec4::new(0.1, 0.1, 0.15, 0.8)
        };
        renderer.draw_rect(self.position, self.size, header_color);
        
        // Selected text
        if let Some(text) = self.options.get(self.selected_index) {
            renderer.draw_text(text, self.position + Vec2::new(10.0, 5.0), 16.0, Vec4::ONE);
        }
        
        // Arrow indicator
        let arrow = if self.expanded { "▲" } else { "▼" };
        renderer.draw_text(arrow, self.position + Vec2::new(self.size.x - 25.0, 5.0), 14.0, Vec4::new(0.7, 0.7, 0.7, 1.0));
        
        // Dropdown panel
        if self.expanded {
            let dropdown_pos = self.position + Vec2::new(0.0, self.size.y);
            renderer.draw_rect(dropdown_pos, Vec2::new(self.size.x, self.dropdown_height()), Vec4::new(0.12, 0.12, 0.18, 0.95));
            
            for (i, option) in self.options.iter().enumerate() {
                let item_pos = dropdown_pos + Vec2::new(0.0, i as f32 * self.item_height);
                
                // Hover or selected highlight
                if self.hovered_index == Some(i) {
                    renderer.draw_rect(item_pos, Vec2::new(self.size.x, self.item_height), Vec4::new(0.2, 0.4, 0.6, 0.8));
                } else if i == self.selected_index {
                    renderer.draw_rect(item_pos, Vec2::new(self.size.x, self.item_height), Vec4::new(0.15, 0.25, 0.35, 0.5));
                }
                
                renderer.draw_text(option, item_pos + Vec2::new(10.0, 5.0), 16.0, Vec4::ONE);
            }
        }
    }
}

// --- TabBar ---

pub struct TabBar {
    pub position: Vec2,
    pub size: Vec2,
    pub tabs: Vec<String>,
    pub selected_index: usize,
    pub children: Vec<Box<dyn Widget>>,
    pub tab_width: f32,
    pub tab_height: f32,
    pub hovered_index: Option<usize>,
}

impl TabBar {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            tabs: Vec::new(),
            selected_index: 0,
            children: Vec::new(),
            tab_width: 120.0,
            tab_height: 35.0,
            hovered_index: None,
        }
    }
    
    pub fn add_tab(mut self, name: &str, content: Box<dyn Widget>) -> Self {
        self.tabs.push(name.to_string());
        self.children.push(content);
        self
    }
}

impl Widget for TabBar {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;
        self.size = max_size;
        
        // Layout only the selected child
        let content_origin = origin + Vec2::new(0.0, self.tab_height);
        let content_size = Vec2::new(max_size.x, max_size.y - self.tab_height);
        
        if let Some(child) = self.children.get_mut(self.selected_index) {
            child.layout(content_origin, content_size);
        }
        
        self.size
    }

    fn handle_event(&mut self, event: &winit::event::Event<()>, mouse_pos: Vec2) -> bool {
        // Tab bar hit detection
        let tab_bar_min = self.position;
        let tab_bar_max = self.position + Vec2::new(self.tabs.len() as f32 * self.tab_width, self.tab_height);
        let in_tabs = mouse_pos.x >= tab_bar_min.x && mouse_pos.x <= tab_bar_max.x &&
                      mouse_pos.y >= tab_bar_min.y && mouse_pos.y <= tab_bar_max.y;
        
        // Calculate hovered tab
        if in_tabs {
            let relative_x = mouse_pos.x - self.position.x;
            let index = (relative_x / self.tab_width) as usize;
            if index < self.tabs.len() {
                self.hovered_index = Some(index);
            } else {
                self.hovered_index = None;
            }
        } else {
            self.hovered_index = None;
        }
        
        // Tab click
        match event {
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } => {
                if in_tabs {
                    if let Some(index) = self.hovered_index {
                        if index != self.selected_index {
                            self.selected_index = index;
                            println!("Tab selected: {}", self.tabs[index]);
                        }
                        return true;
                    }
                }
            },
            _ => {}
        }
        
        // Pass to selected child
        if let Some(child) = self.children.get_mut(self.selected_index) {
            if child.handle_event(event, mouse_pos) {
                return true;
            }
        }
        
        false
    }

    fn update(&mut self, dt: f32) {
        if let Some(child) = self.children.get_mut(self.selected_index) {
            child.update(dt);
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        // Tab bar background
        renderer.draw_rect(self.position, Vec2::new(self.size.x, self.tab_height), Vec4::new(0.05, 0.05, 0.08, 0.9));
        
        // Tabs
        for (i, tab) in self.tabs.iter().enumerate() {
            let tab_pos = self.position + Vec2::new(i as f32 * self.tab_width, 0.0);
            
            let color = if i == self.selected_index {
                Vec4::new(0.15, 0.15, 0.2, 1.0)
            } else if self.hovered_index == Some(i) {
                Vec4::new(0.1, 0.1, 0.15, 0.8)
            } else {
                Vec4::new(0.07, 0.07, 0.1, 0.6)
            };
            
            renderer.draw_rect(tab_pos, Vec2::new(self.tab_width - 2.0, self.tab_height), color);
            
            // Active indicator
            if i == self.selected_index {
                renderer.draw_rect(
                    tab_pos + Vec2::new(0.0, self.tab_height - 3.0),
                    Vec2::new(self.tab_width - 2.0, 3.0),
                    Vec4::new(0.0, 0.8, 1.0, 1.0)
                );
            }
            
            // Tab text
            renderer.draw_text(tab, tab_pos + Vec2::new(15.0, 8.0), 14.0, Vec4::new(1.0, 1.0, 1.0, 0.9));
        }
        
        // Content area background
        let content_pos = self.position + Vec2::new(0.0, self.tab_height);
        let content_size = Vec2::new(self.size.x, self.size.y - self.tab_height);
        renderer.draw_rect(content_pos, content_size, Vec4::new(0.08, 0.08, 0.1, 0.8));
        
        // Render selected child
        if let Some(child) = self.children.get(self.selected_index) {
            child.render(renderer);
        }
    }
}

