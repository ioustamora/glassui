//! GlassUI Sound System
//!
//! Audio feedback for UI interactions:
//! - Click sounds
//! - Notification sounds
//! - Ambient sounds
//! - Spatial audio (panel-aware)

use std::path::PathBuf;

// =============================================================================
// SOUND TYPE
// =============================================================================

/// Types of UI sounds
#[derive(Clone, Debug, PartialEq)]
pub enum UiSound {
    // Interaction sounds
    Click,
    Hover,
    Select,
    Deselect,
    
    // Action sounds
    Success,
    Error,
    Warning,
    Info,
    
    // Task sounds
    TaskStart,
    TaskComplete,
    TaskProgress,
    
    // Panel sounds
    PanelOpen,
    PanelClose,
    PanelSnap,
    PanelResize,
    
    // Chat sounds
    MessageSent,
    MessageReceived,
    Typing,
    
    // System sounds
    Notification,
    Alert,
    Chime,
    
    // Custom sound file
    Custom(PathBuf),
}

impl UiSound {
    /// Get the sound file name (would be in assets/sounds/)
    pub fn filename(&self) -> &'static str {
        match self {
            UiSound::Click => "click.wav",
            UiSound::Hover => "hover.wav",
            UiSound::Select => "select.wav",
            UiSound::Deselect => "deselect.wav",
            UiSound::Success => "success.wav",
            UiSound::Error => "error.wav",
            UiSound::Warning => "warning.wav",
            UiSound::Info => "info.wav",
            UiSound::TaskStart => "task_start.wav",
            UiSound::TaskComplete => "task_complete.wav",
            UiSound::TaskProgress => "task_progress.wav",
            UiSound::PanelOpen => "panel_open.wav",
            UiSound::PanelClose => "panel_close.wav",
            UiSound::PanelSnap => "panel_snap.wav",
            UiSound::PanelResize => "panel_resize.wav",
            UiSound::MessageSent => "message_sent.wav",
            UiSound::MessageReceived => "message_received.wav",
            UiSound::Typing => "typing.wav",
            UiSound::Notification => "notification.wav",
            UiSound::Alert => "alert.wav",
            UiSound::Chime => "chime.wav",
            UiSound::Custom(_) => "",
        }
    }
    
    /// Get default volume (0.0 to 1.0)
    pub fn default_volume(&self) -> f32 {
        match self {
            UiSound::Hover | UiSound::Typing => 0.3,
            UiSound::Click | UiSound::Select => 0.5,
            UiSound::Notification | UiSound::Alert => 0.8,
            _ => 0.6,
        }
    }
}

// =============================================================================
// SOUND SETTINGS
// =============================================================================

/// Sound playback settings
#[derive(Clone, Debug)]
pub struct SoundSettings {
    /// Master volume (0.0 to 1.0)
    pub master_volume: f32,
    /// UI sounds enabled
    pub ui_sounds_enabled: bool,
    /// Notification sounds enabled
    pub notifications_enabled: bool,
    /// Ambient sounds enabled
    pub ambient_enabled: bool,
    /// Spatial audio enabled
    pub spatial_enabled: bool,
}

impl Default for SoundSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.7,
            ui_sounds_enabled: true,
            notifications_enabled: true,
            ambient_enabled: false,
            spatial_enabled: true,
        }
    }
}

// =============================================================================
// SOUND MANAGER
// =============================================================================

/// Manages sound playback
/// Note: Actual playback requires rodio crate
pub struct SoundManager {
    pub settings: SoundSettings,
    queue: Vec<QueuedSound>,
}

#[derive(Clone, Debug)]
struct QueuedSound {
    sound: UiSound,
    volume: f32,
    pan: f32,  // -1.0 (left) to 1.0 (right) for spatial
}

impl SoundManager {
    pub fn new() -> Self {
        Self {
            settings: SoundSettings::default(),
            queue: Vec::new(),
        }
    }
    
    /// Play a UI sound
    pub fn play(&mut self, sound: UiSound) {
        if !self.settings.ui_sounds_enabled {
            return;
        }
        
        let volume = sound.default_volume() * self.settings.master_volume;
        self.queue.push(QueuedSound {
            sound,
            volume,
            pan: 0.0,
        });
    }
    
    /// Play a sound with spatial positioning
    /// x_position: 0.0 (left edge) to 1.0 (right edge)
    pub fn play_spatial(&mut self, sound: UiSound, x_position: f32) {
        if !self.settings.ui_sounds_enabled || !self.settings.spatial_enabled {
            self.play(sound);
            return;
        }
        
        let volume = sound.default_volume() * self.settings.master_volume;
        let pan = (x_position - 0.5) * 2.0;  // Convert to -1.0 to 1.0
        
        self.queue.push(QueuedSound {
            sound,
            volume,
            pan: pan.clamp(-1.0, 1.0),
        });
    }
    
    /// Play notification sound
    pub fn notify(&mut self, sound: UiSound) {
        if !self.settings.notifications_enabled {
            return;
        }
        
        let volume = sound.default_volume() * self.settings.master_volume;
        self.queue.push(QueuedSound {
            sound,
            volume,
            pan: 0.0,
        });
    }
    
    /// Process the sound queue (call each frame)
    /// In a real implementation, this would use rodio to play sounds
    pub fn process(&mut self) {
        // TODO: Implement actual sound playback with rodio
        // For now, just clear the queue
        self.queue.clear();
    }
    
    /// Get pending sound count (for debugging)
    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }
}

impl Default for SoundManager {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// SOUND FEEDBACK TRAIT
// =============================================================================

/// Trait for widgets that provide sound feedback
pub trait SoundFeedback {
    /// Get the sound to play on hover
    fn hover_sound(&self) -> Option<UiSound> {
        Some(UiSound::Hover)
    }
    
    /// Get the sound to play on click
    fn click_sound(&self) -> Option<UiSound> {
        Some(UiSound::Click)
    }
    
    /// Get the sound to play on action completion
    fn action_sound(&self) -> Option<UiSound> {
        None
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sound_manager() {
        let mut manager = SoundManager::new();
        
        manager.play(UiSound::Click);
        manager.play(UiSound::Success);
        
        assert_eq!(manager.pending_count(), 2);
        
        manager.process();
        assert_eq!(manager.pending_count(), 0);
    }
    
    #[test]
    fn test_spatial_sound() {
        let mut manager = SoundManager::new();
        
        manager.play_spatial(UiSound::PanelSnap, 0.25);  // Left side
        manager.play_spatial(UiSound::PanelSnap, 0.75);  // Right side
        
        assert_eq!(manager.pending_count(), 2);
    }
}
