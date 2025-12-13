//! GlassUI Task System
//!
//! Provides async task management with:
//! - Hybrid thread/async execution
//! - Task status tracking
//! - Notification sounds
//! - Inter-task communication

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use std::path::PathBuf;

use crate::widget_id::WidgetId;
use crate::panel_style::PanelPreset;

// =============================================================================
// TASK ID
// =============================================================================

/// Unique identifier for a background task
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

static TASK_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

impl TaskId {
    pub fn new() -> Self {
        Self(TASK_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
    
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TASK STATUS
// =============================================================================

/// Status of a background task
#[derive(Clone, Debug, PartialEq)]
pub enum TaskStatus {
    /// Task is queued but not started
    Pending,
    /// Task is currently running
    Running,
    /// Task is paused
    Paused,
    /// Task completed successfully
    Completed,
    /// Task failed with error
    Failed(String),
    /// Task was cancelled
    Cancelled,
}

impl TaskStatus {
    pub fn is_finished(&self) -> bool {
        matches!(self, TaskStatus::Completed | TaskStatus::Failed(_) | TaskStatus::Cancelled)
    }
    
    pub fn is_running(&self) -> bool {
        matches!(self, TaskStatus::Running)
    }
}

// =============================================================================
// NOTIFICATION SOUND
// =============================================================================

/// Sound to play on task events
#[derive(Clone, Debug)]
pub enum NotificationSound {
    /// No sound
    None,
    /// Short ping
    Ping,
    /// Chime sound
    Chime,
    /// Alert sound
    Alert,
    /// Success fanfare
    Success,
    /// Error sound
    Error,
    /// Custom sound file
    Custom(PathBuf),
}

impl Default for NotificationSound {
    fn default() -> Self {
        NotificationSound::Ping
    }
}

// =============================================================================
// TASK
// =============================================================================

/// A background task with progress tracking
#[derive(Clone, Debug)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub status: TaskStatus,
    pub progress: f32,
    pub started_at: Option<Instant>,
    pub completed_at: Option<Instant>,
    pub on_complete_sound: NotificationSound,
    pub on_error_sound: NotificationSound,
    pub panel_preset: PanelPreset,
}

impl Task {
    /// Create a new task
    pub fn new(name: &str) -> Self {
        Self {
            id: TaskId::new(),
            name: name.to_string(),
            status: TaskStatus::Pending,
            progress: 0.0,
            started_at: None,
            completed_at: None,
            on_complete_sound: NotificationSound::Success,
            on_error_sound: NotificationSound::Error,
            panel_preset: PanelPreset::Default,
        }
    }
    
    /// Set the visual preset for this task's panel
    pub fn with_preset(mut self, preset: PanelPreset) -> Self {
        self.panel_preset = preset;
        self
    }
    
    /// Set completion sound
    pub fn with_complete_sound(mut self, sound: NotificationSound) -> Self {
        self.on_complete_sound = sound;
        self
    }
    
    /// Set error sound
    pub fn with_error_sound(mut self, sound: NotificationSound) -> Self {
        self.on_error_sound = sound;
        self
    }
    
    /// Start the task
    pub fn start(&mut self) {
        self.status = TaskStatus::Running;
        self.started_at = Some(Instant::now());
    }
    
    /// Update progress (0.0 to 1.0)
    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }
    
    /// Mark as completed
    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
        self.progress = 1.0;
        self.completed_at = Some(Instant::now());
    }
    
    /// Mark as failed
    pub fn fail(&mut self, error: &str) {
        self.status = TaskStatus::Failed(error.to_string());
        self.completed_at = Some(Instant::now());
    }
    
    /// Cancel the task
    pub fn cancel(&mut self) {
        self.status = TaskStatus::Cancelled;
        self.completed_at = Some(Instant::now());
    }
    
    /// Get elapsed time
    pub fn elapsed(&self) -> Option<Duration> {
        self.started_at.map(|start| {
            self.completed_at.unwrap_or_else(Instant::now).duration_since(start)
        })
    }
}

// =============================================================================
// TASK PANEL
// =============================================================================

/// Visual panel for a task (all tasks must be visible)
#[derive(Clone, Debug)]
pub struct TaskPanel {
    pub widget_id: WidgetId,
    pub task: Task,
    /// Always true - no hidden tasks per user requirement
    pub visible: bool,
    pub minimized: bool,
}

impl TaskPanel {
    pub fn new(task: Task) -> Self {
        Self {
            widget_id: WidgetId::new(),
            task,
            visible: true,  // Always visible
            minimized: false,
        }
    }
    
    /// Minimize the panel (but keep visible in taskbar)
    pub fn minimize(&mut self) {
        self.minimized = true;
    }
    
    /// Restore from minimized
    pub fn restore(&mut self) {
        self.minimized = false;
    }
}

// =============================================================================
// TASK MANAGER
// =============================================================================

/// Manages all active tasks
#[derive(Default)]
pub struct TaskManager {
    tasks: Vec<TaskPanel>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    
    /// Add a new task
    pub fn add(&mut self, task: Task) -> TaskId {
        let id = task.id;
        self.tasks.push(TaskPanel::new(task));
        id
    }
    
    /// Get a task by ID
    pub fn get(&self, id: TaskId) -> Option<&TaskPanel> {
        self.tasks.iter().find(|t| t.task.id == id)
    }
    
    /// Get a mutable reference to a task
    pub fn get_mut(&mut self, id: TaskId) -> Option<&mut TaskPanel> {
        self.tasks.iter_mut().find(|t| t.task.id == id)
    }
    
    /// Get all tasks
    pub fn all(&self) -> &[TaskPanel] {
        &self.tasks
    }
    
    /// Get running tasks
    pub fn running(&self) -> Vec<&TaskPanel> {
        self.tasks.iter().filter(|t| t.task.status.is_running()).collect()
    }
    
    /// Get completed tasks
    pub fn completed(&self) -> Vec<&TaskPanel> {
        self.tasks.iter().filter(|t| t.task.status.is_finished()).collect()
    }
    
    /// Remove completed tasks
    pub fn clear_completed(&mut self) {
        self.tasks.retain(|t| !t.task.status.is_finished());
    }
    
    /// Count active tasks
    pub fn active_count(&self) -> usize {
        self.tasks.iter().filter(|t| !t.task.status.is_finished()).count()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_lifecycle() {
        let mut task = Task::new("Test Task");
        assert_eq!(task.status, TaskStatus::Pending);
        
        task.start();
        assert!(task.status.is_running());
        
        task.set_progress(0.5);
        assert_eq!(task.progress, 0.5);
        
        task.complete();
        assert!(task.status.is_finished());
        assert_eq!(task.progress, 1.0);
    }
    
    #[test]
    fn test_task_manager() {
        let mut manager = TaskManager::new();
        
        let task1 = Task::new("Task 1");
        let task2 = Task::new("Task 2");
        
        let id1 = manager.add(task1);
        let id2 = manager.add(task2);
        
        assert_eq!(manager.all().len(), 2);
        
        manager.get_mut(id1).unwrap().task.start();
        assert_eq!(manager.running().len(), 1);
        
        manager.get_mut(id1).unwrap().task.complete();
        assert_eq!(manager.completed().len(), 1);
    }
}
