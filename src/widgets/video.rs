//! GlassUI Video Player Widget
//!
//! Complete video playback UI with controls.

use glam::{Vec2, Vec4};
use crate::renderer::GlassRenderer;
use crate::layout::{BoxConstraints, Size, Offset};
use super::core::{Widget, get_theme};
use crate::video::{VideoDecoder, MockVideoDecoder, VideoSource, VideoMetadata, PlaybackState};
use winit::event::{Event, WindowEvent, MouseButton, ElementState};

// =============================================================================
// VIDEO PLAYER WIDGET
// =============================================================================

/// Video player widget with playback controls
///
/// # Example
/// ```rust
/// let player = VideoPlayer::new(VideoSource::File("video.mp4".into()))
///     .with_controls(true)
///     .with_autoplay(false);
/// ```
pub struct VideoPlayer {
    // Layout
    pub position: Vec2,
    pub size: Vec2,
    /// Fixed width (None = fill available)
    pub width: Option<f32>,
    /// Fixed height (None = maintain aspect ratio)
    pub height: Option<f32>,

    // Video state
    source: VideoSource,
    metadata: Option<VideoMetadata>,
    decoder: Box<dyn VideoDecoder>,
    state: PlaybackState,
    error: Option<String>,

    // Playback
    current_time: f64,
    volume: f32,
    muted: bool,
    loop_playback: bool,
    autoplay: bool,

    // UI state
    show_controls: bool,
    controls_visible: bool,
    controls_fade_timer: f32,
    hovered: bool,
    seeking: bool,
    seek_position: f64,

    // Control button states
    play_button_hovered: bool,
    seek_bar_hovered: bool,
    volume_hovered: bool,

    // Cached frame for rendering
    frame_color: Vec4, // Simplified: just show a color based on frame
}

impl VideoPlayer {
    pub fn new(source: VideoSource) -> Self {
        let mut player = Self {
            position: Vec2::ZERO,
            size: Vec2::new(640.0, 360.0),
            width: None,
            height: None,
            source: source.clone(),
            metadata: None,
            decoder: Box::new(MockVideoDecoder::new()),
            state: PlaybackState::Stopped,
            error: None,
            current_time: 0.0,
            volume: 1.0,
            muted: false,
            loop_playback: false,
            autoplay: false,
            show_controls: true,
            controls_visible: true,
            controls_fade_timer: 3.0,
            hovered: false,
            seeking: false,
            seek_position: 0.0,
            play_button_hovered: false,
            seek_bar_hovered: false,
            volume_hovered: false,
            frame_color: Vec4::new(0.1, 0.1, 0.12, 1.0),
        };

        // Open the source
        player.open(source);
        player
    }

    /// Open a new video source
    pub fn open(&mut self, source: VideoSource) {
        self.source = source.clone();
        self.state = PlaybackState::Stopped;
        self.error = None;
        self.current_time = 0.0;

        match self.decoder.open(&source) {
            Ok(metadata) => {
                self.metadata = Some(metadata);
                if self.autoplay {
                    self.play();
                }
            }
            Err(e) => {
                self.error = Some(e.to_string());
                self.state = PlaybackState::Error;
            }
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn with_controls(mut self, show: bool) -> Self {
        self.show_controls = show;
        self
    }

    pub fn with_autoplay(mut self, autoplay: bool) -> Self {
        self.autoplay = autoplay;
        if autoplay && self.metadata.is_some() {
            self.play();
        }
        self
    }

    pub fn with_loop(mut self, loop_playback: bool) -> Self {
        self.loop_playback = loop_playback;
        self
    }

    /// Start playback
    pub fn play(&mut self) {
        if self.metadata.is_some() {
            self.state = PlaybackState::Playing;
            self.decoder.set_playing(true);
        }
    }

    /// Pause playback
    pub fn pause(&mut self) {
        self.state = PlaybackState::Paused;
        self.decoder.set_playing(false);
    }

    /// Toggle play/pause
    pub fn toggle_playback(&mut self) {
        match self.state {
            PlaybackState::Playing => self.pause(),
            PlaybackState::Paused | PlaybackState::Stopped => self.play(),
            PlaybackState::Finished => {
                self.seek(0.0);
                self.play();
            }
            _ => {}
        }
    }

    /// Seek to position (in seconds)
    pub fn seek(&mut self, position: f64) {
        let duration = self.duration();
        let clamped = position.clamp(0.0, duration);
        if let Err(e) = self.decoder.seek(clamped) {
            self.error = Some(e.to_string());
        }
        self.current_time = clamped;
    }

    /// Get current position in seconds
    pub fn position_time(&self) -> f64 {
        self.decoder.position()
    }

    /// Get total duration in seconds
    pub fn duration(&self) -> f64 {
        self.metadata.as_ref().map(|m| m.duration).unwrap_or(0.0)
    }

    /// Get playback progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        let duration = self.duration();
        if duration > 0.0 {
            (self.decoder.position() / duration) as f32
        } else {
            0.0
        }
    }

    /// Set volume (0.0 to 1.0)
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    /// Toggle mute
    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }

    /// Check if point is inside player bounds
    fn contains(&self, point: Vec2) -> bool {
        point.x >= self.position.x
            && point.x <= self.position.x + self.size.x
            && point.y >= self.position.y
            && point.y <= self.position.y + self.size.y
    }

    /// Check if point is inside control bar
    fn contains_controls(&self, point: Vec2) -> bool {
        if !self.show_controls || !self.controls_visible {
            return false;
        }
        let controls_height = 50.0;
        let controls_y = self.position.y + self.size.y - controls_height;
        point.x >= self.position.x
            && point.x <= self.position.x + self.size.x
            && point.y >= controls_y
            && point.y <= self.position.y + self.size.y
    }

    /// Check if point is on play button
    fn contains_play_button(&self, point: Vec2) -> bool {
        let btn_x = self.position.x + 10.0;
        let btn_y = self.position.y + self.size.y - 40.0;
        let btn_size = 30.0;
        point.x >= btn_x && point.x <= btn_x + btn_size
            && point.y >= btn_y && point.y <= btn_y + btn_size
    }

    /// Check if point is on seek bar
    fn contains_seek_bar(&self, point: Vec2) -> bool {
        let bar_x = self.position.x + 50.0;
        let bar_y = self.position.y + self.size.y - 30.0;
        let bar_width = self.size.x - 150.0;
        let bar_height = 10.0;
        point.x >= bar_x && point.x <= bar_x + bar_width
            && point.y >= bar_y - 5.0 && point.y <= bar_y + bar_height + 5.0
    }

    /// Get seek position from mouse x
    fn seek_position_from_x(&self, x: f32) -> f64 {
        let bar_x = self.position.x + 50.0;
        let bar_width = self.size.x - 150.0;
        let relative = ((x - bar_x) / bar_width).clamp(0.0, 1.0);
        relative as f64 * self.duration()
    }

    /// Format time as MM:SS
    fn format_time(seconds: f64) -> String {
        let mins = (seconds / 60.0) as u32;
        let secs = (seconds % 60.0) as u32;
        format!("{:02}:{:02}", mins, secs)
    }
}

impl Widget for VideoPlayer {
    fn layout(&mut self, origin: Vec2, max_size: Vec2) -> Vec2 {
        self.position = origin;

        // Determine size based on aspect ratio
        let aspect = self.metadata.as_ref()
            .map(|m| m.width as f32 / m.height as f32)
            .unwrap_or(16.0 / 9.0);

        let width = self.width.unwrap_or(max_size.x);
        let height = self.height.unwrap_or(width / aspect);

        self.size = Vec2::new(
            width.min(max_size.x),
            height.min(max_size.y),
        );

        self.size
    }

    fn layout_with_constraints(&mut self, constraints: BoxConstraints) -> Size {
        let aspect = self.metadata.as_ref()
            .map(|m| m.width as f32 / m.height as f32)
            .unwrap_or(16.0 / 9.0);

        let width = self.width.unwrap_or(constraints.max_width);
        let height = self.height.unwrap_or(width / aspect);

        let size = constraints.constrain(Size::new(width, height));
        self.size = Vec2::new(size.width, size.height);
        size
    }

    fn set_position(&mut self, position: Offset) {
        self.position = Vec2::new(position.x, position.y);
    }

    fn get_position(&self) -> Offset {
        Offset::new(self.position.x, self.position.y)
    }

    fn get_size(&self) -> Size {
        Size::new(self.size.x, self.size.y)
    }

    fn intrinsic_width(&self, _height: f32) -> Option<f32> {
        self.width.or_else(|| {
            self.metadata.as_ref().map(|m| m.width as f32)
        })
    }

    fn intrinsic_height(&self, _width: f32) -> Option<f32> {
        self.height.or_else(|| {
            self.metadata.as_ref().map(|m| m.height as f32)
        })
    }

    fn handle_event(&mut self, event: &Event<()>, mouse_pos: Vec2) -> bool {
        // Track hover state
        self.hovered = self.contains(mouse_pos);
        self.play_button_hovered = self.contains_play_button(mouse_pos);
        self.seek_bar_hovered = self.contains_seek_bar(mouse_pos);

        if self.hovered {
            self.controls_visible = true;
            self.controls_fade_timer = 3.0;
        }

        match event {
            Event::WindowEvent { event: WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, .. } => {
                if self.play_button_hovered {
                    self.toggle_playback();
                    return true;
                }
                if self.seek_bar_hovered {
                    self.seeking = true;
                    let new_pos = self.seek_position_from_x(mouse_pos.x);
                    self.seek(new_pos);
                    return true;
                }
                // Click on video area toggles playback
                if self.hovered && !self.contains_controls(mouse_pos) {
                    self.toggle_playback();
                    return true;
                }
            }
            Event::WindowEvent { event: WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. }, .. } => {
                if self.seeking {
                    self.seeking = false;
                    return true;
                }
            }
            Event::WindowEvent { event: WindowEvent::CursorMoved { .. }, .. } => {
                if self.seeking {
                    let new_pos = self.seek_position_from_x(mouse_pos.x);
                    self.seek(new_pos);
                    return true;
                }
            }
            _ => {}
        }

        false
    }

    fn update(&mut self, dt: f32) {
        // Update decoder
        self.decoder.update(dt);

        // Update current time
        self.current_time = self.decoder.position();

        // Check if finished
        if self.decoder.is_finished() {
            if self.loop_playback {
                self.decoder.reset();
                self.decoder.set_playing(true);
            } else {
                self.state = PlaybackState::Finished;
                self.decoder.set_playing(false);
            }
        }

        // Decode frame and update display color
        if let Ok(Some(frame)) = self.decoder.decode_frame() {
            // Sample the center pixel for display color (simplified visualization)
            let idx = ((frame.height / 2) * frame.width + frame.width / 2) as usize * 4;
            if idx + 3 < frame.data.len() {
                self.frame_color = Vec4::new(
                    frame.data[idx] as f32 / 255.0,
                    frame.data[idx + 1] as f32 / 255.0,
                    frame.data[idx + 2] as f32 / 255.0,
                    1.0,
                );
            }
        }

        // Fade controls when not hovered
        if self.show_controls && !self.hovered && self.state == PlaybackState::Playing {
            self.controls_fade_timer -= dt;
            if self.controls_fade_timer <= 0.0 {
                self.controls_visible = false;
            }
        }
    }

    fn render(&self, renderer: &mut GlassRenderer) {
        let theme = get_theme();

        // Draw video frame area
        renderer.draw_rounded_rect(
            self.position,
            self.size,
            self.frame_color,
            8.0,
        );

        // Draw error state
        if let Some(ref error) = self.error {
            let center = self.position + self.size / 2.0;
            renderer.draw_text("⚠", center - Vec2::new(15.0, 20.0), 30.0, theme.error);
            renderer.draw_text(error, center + Vec2::new(-50.0, 20.0), 12.0, theme.text_secondary);
            return;
        }

        // Draw loading state
        if self.state == PlaybackState::Buffering {
            let center = self.position + self.size / 2.0;
            renderer.draw_text("⏳", center - Vec2::new(15.0, 15.0), 30.0, theme.text);
        }

        // Draw paused/stopped overlay
        if self.state == PlaybackState::Paused || self.state == PlaybackState::Stopped {
            let center = self.position + self.size / 2.0;
            let icon_bg_pos = center - Vec2::new(30.0, 30.0);
            renderer.draw_rounded_rect(
                icon_bg_pos,
                Vec2::new(60.0, 60.0),
                Vec4::new(0.0, 0.0, 0.0, 0.5),
                30.0,
            );
            renderer.draw_text("▶", center - Vec2::new(12.0, 15.0), 30.0, theme.text);
        }

        // Draw controls
        if self.show_controls && self.controls_visible {
            let controls_height = 50.0;
            let controls_y = self.position.y + self.size.y - controls_height;

            // Control bar background
            renderer.draw_rounded_rect(
                Vec2::new(self.position.x, controls_y),
                Vec2::new(self.size.x, controls_height),
                Vec4::new(0.0, 0.0, 0.0, 0.7),
                0.0,
            );

            // Play/Pause button
            let btn_x = self.position.x + 10.0;
            let btn_y = controls_y + 10.0;
            let btn_color = if self.play_button_hovered {
                theme.primary
            } else {
                theme.text
            };
            let play_icon = if self.state == PlaybackState::Playing { "⏸" } else { "▶" };
            renderer.draw_text(play_icon, Vec2::new(btn_x, btn_y), 24.0, btn_color);

            // Seek bar
            let bar_x = self.position.x + 50.0;
            let bar_y = controls_y + 20.0;
            let bar_width = self.size.x - 150.0;
            let bar_height = 6.0;

            // Background track
            renderer.draw_rounded_rect(
                Vec2::new(bar_x, bar_y),
                Vec2::new(bar_width, bar_height),
                Vec4::new(0.3, 0.3, 0.35, 0.8),
                3.0,
            );

            // Progress fill
            let progress_width = bar_width * self.progress();
            if progress_width > 0.0 {
                renderer.draw_rounded_rect(
                    Vec2::new(bar_x, bar_y),
                    Vec2::new(progress_width, bar_height),
                    theme.primary,
                    3.0,
                );
            }

            // Seek handle
            let handle_x = bar_x + progress_width - 6.0;
            let handle_color = if self.seek_bar_hovered || self.seeking {
                theme.primary
            } else {
                theme.text
            };
            renderer.draw_rounded_rect(
                Vec2::new(handle_x, bar_y - 3.0),
                Vec2::new(12.0, 12.0),
                handle_color,
                6.0,
            );

            // Time display
            let time_x = self.position.x + self.size.x - 90.0;
            let time_text = format!(
                "{} / {}",
                Self::format_time(self.current_time),
                Self::format_time(self.duration())
            );
            renderer.draw_text(&time_text, Vec2::new(time_x, controls_y + 18.0), 12.0, theme.text);
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_player_creation() {
        let player = VideoPlayer::new(VideoSource::default());
        assert_eq!(player.state, PlaybackState::Stopped);
        assert!(player.metadata.is_some());
    }

    #[test]
    fn test_video_player_playback() {
        let mut player = VideoPlayer::new(VideoSource::Mock {
            duration: 10.0,
            width: 640,
            height: 360,
        });

        player.play();
        assert_eq!(player.state, PlaybackState::Playing);

        player.pause();
        assert_eq!(player.state, PlaybackState::Paused);
    }

    #[test]
    fn test_video_player_seek() {
        let mut player = VideoPlayer::new(VideoSource::Mock {
            duration: 60.0,
            width: 640,
            height: 360,
        });

        player.seek(30.0);
        assert!((player.position_time() - 30.0).abs() < 0.1);
    }

    #[test]
    fn test_format_time() {
        assert_eq!(VideoPlayer::format_time(0.0), "00:00");
        assert_eq!(VideoPlayer::format_time(65.0), "01:05");
        assert_eq!(VideoPlayer::format_time(3661.0), "61:01");
    }
}
