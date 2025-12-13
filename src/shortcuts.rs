//! GlassUI Keyboard Shortcuts Manager
//!
//! Global keyboard shortcut handling:
//! - Register shortcuts with callbacks
//! - Modifier key combinations
//! - Context-aware shortcuts

use std::collections::HashMap;

// =============================================================================
// KEY MODIFIERS
// =============================================================================

/// Modifier keys for shortcuts
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,  // Windows/Cmd key
}

impl Modifiers {
    pub fn none() -> Self {
        Self::default()
    }
    
    pub fn ctrl() -> Self {
        Self { ctrl: true, ..Default::default() }
    }
    
    pub fn alt() -> Self {
        Self { alt: true, ..Default::default() }
    }
    
    pub fn shift() -> Self {
        Self { shift: true, ..Default::default() }
    }
    
    pub fn ctrl_shift() -> Self {
        Self { ctrl: true, shift: true, ..Default::default() }
    }
    
    pub fn ctrl_alt() -> Self {
        Self { ctrl: true, alt: true, ..Default::default() }
    }
}

// =============================================================================
// SHORTCUT KEY
// =============================================================================

/// A key that can be part of a shortcut
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ShortcutKey {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    // Numbers
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,
    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    // Special
    Escape, Enter, Space, Tab, Backspace, Delete,
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    Home, End, PageUp, PageDown,
    // Punctuation
    Comma, Period, Slash, Semicolon, Quote,
    BracketLeft, BracketRight, Backslash, Minus, Equal,
}

impl ShortcutKey {
    /// Try to parse from a winit logical key
    pub fn from_logical_key(key: &winit::keyboard::Key) -> Option<Self> {
        match key {
            winit::keyboard::Key::Character(s) => {
                match s.to_lowercase().as_str() {
                    "a" => Some(Self::A), "b" => Some(Self::B), "c" => Some(Self::C),
                    "d" => Some(Self::D), "e" => Some(Self::E), "f" => Some(Self::F),
                    "g" => Some(Self::G), "h" => Some(Self::H), "i" => Some(Self::I),
                    "j" => Some(Self::J), "k" => Some(Self::K), "l" => Some(Self::L),
                    "m" => Some(Self::M), "n" => Some(Self::N), "o" => Some(Self::O),
                    "p" => Some(Self::P), "q" => Some(Self::Q), "r" => Some(Self::R),
                    "s" => Some(Self::S), "t" => Some(Self::T), "u" => Some(Self::U),
                    "v" => Some(Self::V), "w" => Some(Self::W), "x" => Some(Self::X),
                    "y" => Some(Self::Y), "z" => Some(Self::Z),
                    "0" => Some(Self::Key0), "1" => Some(Self::Key1), "2" => Some(Self::Key2),
                    "3" => Some(Self::Key3), "4" => Some(Self::Key4), "5" => Some(Self::Key5),
                    "6" => Some(Self::Key6), "7" => Some(Self::Key7), "8" => Some(Self::Key8),
                    "9" => Some(Self::Key9),
                    "," => Some(Self::Comma), "." => Some(Self::Period),
                    "/" => Some(Self::Slash), ";" => Some(Self::Semicolon),
                    "'" => Some(Self::Quote), "[" => Some(Self::BracketLeft),
                    "]" => Some(Self::BracketRight), "\\" => Some(Self::Backslash),
                    "-" => Some(Self::Minus), "=" => Some(Self::Equal),
                    _ => None,
                }
            },
            winit::keyboard::Key::Named(named) => {
                use winit::keyboard::NamedKey;
                match named {
                    NamedKey::Escape => Some(Self::Escape),
                    NamedKey::Enter => Some(Self::Enter),
                    NamedKey::Space => Some(Self::Space),
                    NamedKey::Tab => Some(Self::Tab),
                    NamedKey::Backspace => Some(Self::Backspace),
                    NamedKey::Delete => Some(Self::Delete),
                    NamedKey::ArrowUp => Some(Self::ArrowUp),
                    NamedKey::ArrowDown => Some(Self::ArrowDown),
                    NamedKey::ArrowLeft => Some(Self::ArrowLeft),
                    NamedKey::ArrowRight => Some(Self::ArrowRight),
                    NamedKey::Home => Some(Self::Home),
                    NamedKey::End => Some(Self::End),
                    NamedKey::PageUp => Some(Self::PageUp),
                    NamedKey::PageDown => Some(Self::PageDown),
                    NamedKey::F1 => Some(Self::F1), NamedKey::F2 => Some(Self::F2),
                    NamedKey::F3 => Some(Self::F3), NamedKey::F4 => Some(Self::F4),
                    NamedKey::F5 => Some(Self::F5), NamedKey::F6 => Some(Self::F6),
                    NamedKey::F7 => Some(Self::F7), NamedKey::F8 => Some(Self::F8),
                    NamedKey::F9 => Some(Self::F9), NamedKey::F10 => Some(Self::F10),
                    NamedKey::F11 => Some(Self::F11), NamedKey::F12 => Some(Self::F12),
                    _ => None,
                }
            },
            _ => None,
        }
    }
    
    /// Get display string for the key
    pub fn display(&self) -> &'static str {
        match self {
            Self::A => "A", Self::B => "B", Self::C => "C", Self::D => "D",
            Self::E => "E", Self::F => "F", Self::G => "G", Self::H => "H",
            Self::I => "I", Self::J => "J", Self::K => "K", Self::L => "L",
            Self::M => "M", Self::N => "N", Self::O => "O", Self::P => "P",
            Self::Q => "Q", Self::R => "R", Self::S => "S", Self::T => "T",
            Self::U => "U", Self::V => "V", Self::W => "W", Self::X => "X",
            Self::Y => "Y", Self::Z => "Z",
            Self::Key0 => "0", Self::Key1 => "1", Self::Key2 => "2",
            Self::Key3 => "3", Self::Key4 => "4", Self::Key5 => "5",
            Self::Key6 => "6", Self::Key7 => "7", Self::Key8 => "8", Self::Key9 => "9",
            Self::F1 => "F1", Self::F2 => "F2", Self::F3 => "F3", Self::F4 => "F4",
            Self::F5 => "F5", Self::F6 => "F6", Self::F7 => "F7", Self::F8 => "F8",
            Self::F9 => "F9", Self::F10 => "F10", Self::F11 => "F11", Self::F12 => "F12",
            Self::Escape => "Esc", Self::Enter => "Enter", Self::Space => "Space",
            Self::Tab => "Tab", Self::Backspace => "Backspace", Self::Delete => "Del",
            Self::ArrowUp => "↑", Self::ArrowDown => "↓",
            Self::ArrowLeft => "←", Self::ArrowRight => "→",
            Self::Home => "Home", Self::End => "End",
            Self::PageUp => "PgUp", Self::PageDown => "PgDn",
            Self::Comma => ",", Self::Period => ".",
            Self::Slash => "/", Self::Semicolon => ";", Self::Quote => "'",
            Self::BracketLeft => "[", Self::BracketRight => "]",
            Self::Backslash => "\\", Self::Minus => "-", Self::Equal => "=",
        }
    }
}

// =============================================================================
// SHORTCUT
// =============================================================================

/// A keyboard shortcut (modifiers + key)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Shortcut {
    pub modifiers: Modifiers,
    pub key: ShortcutKey,
}

impl Shortcut {
    pub fn new(key: ShortcutKey) -> Self {
        Self { modifiers: Modifiers::none(), key }
    }
    
    pub fn ctrl(key: ShortcutKey) -> Self {
        Self { modifiers: Modifiers::ctrl(), key }
    }
    
    pub fn alt(key: ShortcutKey) -> Self {
        Self { modifiers: Modifiers::alt(), key }
    }
    
    pub fn shift(key: ShortcutKey) -> Self {
        Self { modifiers: Modifiers::shift(), key }
    }
    
    pub fn ctrl_shift(key: ShortcutKey) -> Self {
        Self { modifiers: Modifiers::ctrl_shift(), key }
    }
    
    /// Get display string like "Ctrl+K"
    pub fn display(&self) -> String {
        let mut parts = Vec::new();
        if self.modifiers.ctrl { parts.push("Ctrl"); }
        if self.modifiers.alt { parts.push("Alt"); }
        if self.modifiers.shift { parts.push("Shift"); }
        if self.modifiers.meta { parts.push("Meta"); }
        parts.push(self.key.display());
        parts.join("+")
    }
}

// =============================================================================
// SHORTCUT MANAGER
// =============================================================================

/// Action ID for shortcuts
pub type ActionId = String;

/// Registered shortcut with metadata
#[derive(Clone, Debug)]
pub struct RegisteredShortcut {
    pub shortcut: Shortcut,
    pub action_id: ActionId,
    pub description: String,
    pub enabled: bool,
}

/// Manages keyboard shortcuts
pub struct ShortcutManager {
    shortcuts: HashMap<Shortcut, RegisteredShortcut>,
    current_modifiers: Modifiers,
}

impl ShortcutManager {
    pub fn new() -> Self {
        Self {
            shortcuts: HashMap::new(),
            current_modifiers: Modifiers::none(),
        }
    }
    
    /// Register a shortcut
    pub fn register(&mut self, shortcut: Shortcut, action_id: &str, description: &str) {
        self.shortcuts.insert(shortcut.clone(), RegisteredShortcut {
            shortcut,
            action_id: action_id.to_string(),
            description: description.to_string(),
            enabled: true,
        });
    }
    
    /// Unregister a shortcut
    pub fn unregister(&mut self, shortcut: &Shortcut) {
        self.shortcuts.remove(shortcut);
    }
    
    /// Enable/disable a shortcut
    pub fn set_enabled(&mut self, shortcut: &Shortcut, enabled: bool) {
        if let Some(registered) = self.shortcuts.get_mut(shortcut) {
            registered.enabled = enabled;
        }
    }
    
    /// Get all registered shortcuts
    pub fn all_shortcuts(&self) -> Vec<&RegisteredShortcut> {
        self.shortcuts.values().collect()
    }
    
    /// Handle a keyboard event, returns action ID if matched
    pub fn handle_event(&mut self, event: &winit::event::Event<()>) -> Option<ActionId> {
        if let winit::event::Event::WindowEvent { 
            event: winit::event::WindowEvent::KeyboardInput { event: key_event, .. }, 
            .. 
        } = event {
            // Update modifier state
            self.update_modifiers(key_event);
            
            // Only trigger on key press
            if !key_event.state.is_pressed() {
                return None;
            }
            
            // Try to match shortcut
            if let Some(key) = ShortcutKey::from_logical_key(&key_event.logical_key) {
                let shortcut = Shortcut {
                    modifiers: self.current_modifiers,
                    key,
                };
                
                if let Some(registered) = self.shortcuts.get(&shortcut) {
                    if registered.enabled {
                        return Some(registered.action_id.clone());
                    }
                }
            }
        }
        
        None
    }
    
    fn update_modifiers(&mut self, event: &winit::event::KeyEvent) {
        // Check for modifier keys
        use winit::keyboard::{Key, NamedKey};
        match &event.logical_key {
            Key::Named(NamedKey::Control) => {
                self.current_modifiers.ctrl = event.state.is_pressed();
            },
            Key::Named(NamedKey::Alt) => {
                self.current_modifiers.alt = event.state.is_pressed();
            },
            Key::Named(NamedKey::Shift) => {
                self.current_modifiers.shift = event.state.is_pressed();
            },
            Key::Named(NamedKey::Super) => {
                self.current_modifiers.meta = event.state.is_pressed();
            },
            _ => {}
        }
    }
    
    /// Register common dashboard shortcuts
    pub fn register_dashboard_shortcuts(&mut self) {
        self.register(Shortcut::ctrl(ShortcutKey::K), "command_palette", "Open command palette");
        self.register(Shortcut::ctrl(ShortcutKey::N), "new_panel", "New panel");
        self.register(Shortcut::ctrl(ShortcutKey::W), "close_panel", "Close panel");
        self.register(Shortcut::ctrl(ShortcutKey::S), "save_workspace", "Save workspace");
        self.register(Shortcut::ctrl(ShortcutKey::O), "load_workspace", "Load workspace");
        self.register(Shortcut::new(ShortcutKey::F11), "fullscreen", "Toggle fullscreen");
        self.register(Shortcut::new(ShortcutKey::Escape), "deselect", "Deselect / Close");
        self.register(Shortcut::ctrl_shift(ShortcutKey::P), "preferences", "Open preferences");
    }
}

impl Default for ShortcutManager {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shortcut_display() {
        let shortcut = Shortcut::ctrl(ShortcutKey::K);
        assert_eq!(shortcut.display(), "Ctrl+K");
        
        let shortcut2 = Shortcut::ctrl_shift(ShortcutKey::P);
        assert_eq!(shortcut2.display(), "Ctrl+Shift+P");
    }
    
    #[test]
    fn test_register_shortcuts() {
        let mut manager = ShortcutManager::new();
        manager.register_dashboard_shortcuts();
        
        assert!(manager.all_shortcuts().len() >= 5);
    }
}
