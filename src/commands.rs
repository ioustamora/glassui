//! GlassUI Command Pattern & Undo/Redo System
//!
//! Provides undo/redo functionality using the command pattern:
//! - `Command` trait for reversible operations
//! - `CommandHistory` for managing undo/redo stacks
//! - Pre-built commands for common operations

// =============================================================================
// COMMAND TRAIT
// =============================================================================

/// A reversible command that can be executed and undone
pub trait Command: std::fmt::Debug {
    /// Execute the command
    fn execute(&mut self);
    
    /// Undo the command (reverse its effects)
    fn undo(&mut self);
    
    /// Get a description of this command
    fn description(&self) -> &str;
    
    /// Whether this command can be merged with the previous one
    /// (for coalescing rapid changes like typing)
    fn can_merge(&self, _other: &dyn Command) -> bool {
        false
    }
    
    /// Merge another command into this one (if can_merge returns true)
    fn merge(&mut self, _other: Box<dyn Command>) {}
}

// =============================================================================
// COMMAND HISTORY
// =============================================================================

/// Manages undo/redo history with a stack-based approach
pub struct CommandHistory {
    /// Commands that can be undone
    undo_stack: Vec<Box<dyn Command>>,
    /// Commands that can be redone
    redo_stack: Vec<Box<dyn Command>>,
    /// Maximum history size (0 = unlimited)
    max_history: usize,
    /// Whether we're currently in an undo/redo operation
    is_undoing: bool,
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history: 100,
            is_undoing: false,
        }
    }
    
    /// Set maximum history size
    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }
    
    /// Execute a command and add it to history
    pub fn execute(&mut self, mut command: Box<dyn Command>) {
        command.execute();
        
        // Clear redo stack when new command is executed
        self.redo_stack.clear();
        
        // Try to merge with previous command
        if let Some(prev) = self.undo_stack.last_mut() {
            if prev.can_merge(command.as_ref()) {
                prev.merge(command);
                return;
            }
        }
        
        // Add to undo stack
        self.undo_stack.push(command);
        
        // Limit history size
        if self.max_history > 0 && self.undo_stack.len() > self.max_history {
            self.undo_stack.remove(0);
        }
    }
    
    /// Undo the last command
    pub fn undo(&mut self) -> bool {
        if let Some(mut command) = self.undo_stack.pop() {
            self.is_undoing = true;
            command.undo();
            self.is_undoing = false;
            self.redo_stack.push(command);
            true
        } else {
            false
        }
    }
    
    /// Redo the last undone command
    pub fn redo(&mut self) -> bool {
        if let Some(mut command) = self.redo_stack.pop() {
            command.execute();
            self.undo_stack.push(command);
            true
        } else {
            false
        }
    }
    
    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }
    
    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
    
    /// Get description of next undo
    pub fn undo_description(&self) -> Option<&str> {
        self.undo_stack.last().map(|c| c.description())
    }
    
    /// Get description of next redo
    pub fn redo_description(&self) -> Option<&str> {
        self.redo_stack.last().map(|c| c.description())
    }
    
    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
    
    /// Get undo stack size
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }
    
    /// Get redo stack size
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// VALUE CHANGE COMMAND
// =============================================================================

use std::cell::RefCell;
use std::rc::Rc;

/// Command that changes a value and can restore it
pub struct ValueChangeCommand<T: Clone + 'static> {
    target: Rc<RefCell<T>>,
    old_value: T,
    new_value: T,
    description: String,
}

impl<T: Clone + std::fmt::Debug + 'static> ValueChangeCommand<T> {
    pub fn new(target: Rc<RefCell<T>>, new_value: T, description: &str) -> Self {
        let old_value = target.borrow().clone();
        Self {
            target,
            old_value,
            new_value,
            description: description.to_string(),
        }
    }
}

impl<T: Clone + std::fmt::Debug + 'static> Command for ValueChangeCommand<T> {
    fn execute(&mut self) {
        *self.target.borrow_mut() = self.new_value.clone();
    }
    
    fn undo(&mut self) {
        *self.target.borrow_mut() = self.old_value.clone();
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}

impl<T: Clone + std::fmt::Debug> std::fmt::Debug for ValueChangeCommand<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValueChangeCommand")
            .field("old", &self.old_value)
            .field("new", &self.new_value)
            .field("desc", &self.description)
            .finish()
    }
}

// =============================================================================
// TEXT EDIT COMMAND
// =============================================================================

/// Command for text editing operations
#[derive(Debug)]
pub struct TextEditCommand {
    target: Rc<RefCell<String>>,
    old_text: String,
    new_text: String,
    description: String,
    /// Position where the edit occurred
    position: usize,
}

impl TextEditCommand {
    pub fn new(target: Rc<RefCell<String>>, new_text: String, description: &str) -> Self {
        let old_text = target.borrow().clone();
        Self {
            target,
            old_text,
            new_text,
            description: description.to_string(),
            position: 0,
        }
    }
    
    pub fn insert(target: Rc<RefCell<String>>, position: usize, text: &str) -> Self {
        let old_text = target.borrow().clone();
        let mut new_text = old_text.clone();
        if position <= new_text.len() {
            new_text.insert_str(position, text);
        }
        Self {
            target,
            old_text,
            new_text,
            description: format!("Insert \"{}\"", text),
            position,
        }
    }
    
    pub fn delete(target: Rc<RefCell<String>>, start: usize, end: usize) -> Self {
        let old_text = target.borrow().clone();
        let mut new_text = old_text.clone();
        if start < new_text.len() && end <= new_text.len() {
            new_text.replace_range(start..end, "");
        }
        Self {
            target,
            old_text,
            new_text,
            description: "Delete".to_string(),
            position: start,
        }
    }
}

impl Command for TextEditCommand {
    fn execute(&mut self) {
        *self.target.borrow_mut() = self.new_text.clone();
    }
    
    fn undo(&mut self) {
        *self.target.borrow_mut() = self.old_text.clone();
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn can_merge(&self, other: &dyn Command) -> bool {
        // Type check via description pattern (simple approach)
        other.description().starts_with("Insert")
    }
    
    fn merge(&mut self, other: Box<dyn Command>) {
        // For merged inserts, use the other's description if it's an insert
        if other.description().starts_with("Insert") {
            // The text was already applied, just update our new_text
            self.new_text = self.target.borrow().clone();
        }
    }
}

// =============================================================================
// COMPOUND COMMAND
// =============================================================================

/// Groups multiple commands into one undoable action
#[derive(Debug)]
pub struct CompoundCommand {
    commands: Vec<Box<dyn Command>>,
    description: String,
}

impl CompoundCommand {
    pub fn new(description: &str) -> Self {
        Self {
            commands: Vec::new(),
            description: description.to_string(),
        }
    }
    
    pub fn add(mut self, command: Box<dyn Command>) -> Self {
        self.commands.push(command);
        self
    }
}

impl Command for CompoundCommand {
    fn execute(&mut self) {
        for cmd in &mut self.commands {
            cmd.execute();
        }
    }
    
    fn undo(&mut self) {
        // Undo in reverse order
        for cmd in self.commands.iter_mut().rev() {
            cmd.undo();
        }
    }
    
    fn description(&self) -> &str {
        &self.description
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_undo_redo() {
        let value = Rc::new(RefCell::new(10i32));
        let mut history = CommandHistory::new();
        
        // Execute change to 20
        history.execute(Box::new(ValueChangeCommand::new(
            value.clone(), 20, "Set to 20"
        )));
        assert_eq!(*value.borrow(), 20);
        
        // Execute change to 30
        history.execute(Box::new(ValueChangeCommand::new(
            value.clone(), 30, "Set to 30"
        )));
        assert_eq!(*value.borrow(), 30);
        
        // Undo -> 20
        assert!(history.undo());
        assert_eq!(*value.borrow(), 20);
        
        // Undo -> 10
        assert!(history.undo());
        assert_eq!(*value.borrow(), 10);
        
        // Redo -> 20
        assert!(history.redo());
        assert_eq!(*value.borrow(), 20);
    }
    
    #[test]
    fn test_text_edit() {
        let text = Rc::new(RefCell::new(String::from("Hello")));
        let mut history = CommandHistory::new();
        
        // Insert " World"
        history.execute(Box::new(TextEditCommand::insert(
            text.clone(), 5, " World"
        )));
        assert_eq!(*text.borrow(), "Hello World");
        
        // Undo
        history.undo();
        assert_eq!(*text.borrow(), "Hello");
    }
}
