//! GlassUI Video Playback System
//!
//! Provides video decoding abstraction and playback widgets.
//! Uses a trait-based design for pluggable backends (mock, FFmpeg, etc.)
//!
//! # Architecture
//! - `VideoDecoder` trait: Abstract interface for video backends
//! - `MockVideoDecoder`: Development/testing decoder with animated frames
//! - `VideoPlayer` widget: Full playback UI (in widgets/video.rs)
//!
//! # Future Backends
//! - FFmpeg via `ez-ffmpeg` or `rsmpeg`
//! - Hardware-accelerated via `vk-video`

use std::time::Duration;

// =============================================================================
// VIDEO SOURCE
// =============================================================================

/// Source for video content
#[derive(Clone, Debug)]
pub enum VideoSource {
    /// Path to local file
    File(String),
    /// URL for streaming
    Url(String),
    /// Raw video data in memory
    Memory {
        data: Vec<u8>,
        format: String, // e.g., "mp4", "webm"
    },
    /// Placeholder for testing
    Mock {
        duration: f64,
        width: u32,
        height: u32,
    },
}

impl Default for VideoSource {
    fn default() -> Self {
        VideoSource::Mock {
            duration: 10.0,
            width: 640,
            height: 360,
        }
    }
}

// =============================================================================
// VIDEO METADATA
// =============================================================================

/// Metadata about a video
#[derive(Clone, Debug, Default)]
pub struct VideoMetadata {
    /// Duration in seconds
    pub duration: f64,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Frame rate (frames per second)
    pub framerate: f32,
    /// Video codec name
    pub codec: String,
    /// Whether audio is present
    pub has_audio: bool,
    /// Audio codec name (if present)
    pub audio_codec: Option<String>,
}

// =============================================================================
// VIDEO FRAME
// =============================================================================

/// Decoded video frame
#[derive(Clone, Debug)]
pub struct VideoFrame {
    /// Pixel data (RGBA8 format)
    pub data: Vec<u8>,
    /// Frame width
    pub width: u32,
    /// Frame height
    pub height: u32,
    /// Presentation timestamp in seconds
    pub timestamp: f64,
    /// Frame index
    pub frame_number: u64,
}

impl VideoFrame {
    /// Create a solid color frame (for testing)
    pub fn solid(width: u32, height: u32, r: u8, g: u8, b: u8, a: u8, timestamp: f64) -> Self {
        let pixel_count = (width * height) as usize;
        let mut data = Vec::with_capacity(pixel_count * 4);
        for _ in 0..pixel_count {
            data.push(r);
            data.push(g);
            data.push(b);
            data.push(a);
        }
        Self {
            data,
            width,
            height,
            timestamp,
            frame_number: 0,
        }
    }

    /// Create a gradient frame (for testing)
    pub fn gradient(width: u32, height: u32, timestamp: f64) -> Self {
        let pixel_count = (width * height) as usize;
        let mut data = Vec::with_capacity(pixel_count * 4);
        
        for y in 0..height {
            for x in 0..width {
                let r = ((x as f32 / width as f32) * 255.0) as u8;
                let g = ((y as f32 / height as f32) * 255.0) as u8;
                let b = (((timestamp * 50.0) % 255.0)) as u8;
                data.push(r);
                data.push(g);
                data.push(b);
                data.push(255);
            }
        }
        
        Self {
            data,
            width,
            height,
            timestamp,
            frame_number: (timestamp * 30.0) as u64,
        }
    }
}

// =============================================================================
// VIDEO ERROR
// =============================================================================

/// Errors that can occur during video operations
#[derive(Clone, Debug)]
pub enum VideoError {
    /// Failed to open source
    OpenFailed(String),
    /// Unsupported format/codec
    UnsupportedFormat(String),
    /// Decoding error
    DecodeFailed(String),
    /// Seek failed
    SeekFailed(String),
    /// End of stream reached
    EndOfStream,
    /// Generic error
    Other(String),
}

impl std::fmt::Display for VideoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoError::OpenFailed(s) => write!(f, "Failed to open video: {}", s),
            VideoError::UnsupportedFormat(s) => write!(f, "Unsupported format: {}", s),
            VideoError::DecodeFailed(s) => write!(f, "Decode failed: {}", s),
            VideoError::SeekFailed(s) => write!(f, "Seek failed: {}", s),
            VideoError::EndOfStream => write!(f, "End of stream"),
            VideoError::Other(s) => write!(f, "Video error: {}", s),
        }
    }
}

impl std::error::Error for VideoError {}

// =============================================================================
// PLAYBACK STATE
// =============================================================================

/// Current state of video playback
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlaybackState {
    /// Not started or stopped
    #[default]
    Stopped,
    /// Currently playing
    Playing,
    /// Paused
    Paused,
    /// Buffering (loading data)
    Buffering,
    /// Playback finished
    Finished,
    /// Error occurred
    Error,
}

impl PlaybackState {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Playing | Self::Buffering)
    }
}

// =============================================================================
// VIDEO DECODER TRAIT
// =============================================================================

/// Abstract interface for video decoders
/// 
/// Implement this trait to add new video backends (FFmpeg, hardware, etc.)
pub trait VideoDecoder: Send {
    /// Open a video source and return metadata
    fn open(&mut self, source: &VideoSource) -> Result<VideoMetadata, VideoError>;
    
    /// Decode and return the next frame
    /// Returns None if no frame is available yet (buffering)
    fn decode_frame(&mut self) -> Result<Option<VideoFrame>, VideoError>;
    
    /// Seek to a position (in seconds)
    fn seek(&mut self, position: f64) -> Result<(), VideoError>;
    
    /// Get current position in seconds
    fn position(&self) -> f64;
    
    /// Get total duration in seconds
    fn duration(&self) -> f64;
    
    /// Set playback state
    fn set_playing(&mut self, playing: bool);
    
    /// Check if currently playing
    fn is_playing(&self) -> bool;
    
    /// Update decoder state (call each frame)
    fn update(&mut self, dt: f32);
    
    /// Check if decoder has reached end
    fn is_finished(&self) -> bool;
    
    /// Reset to beginning
    fn reset(&mut self);
}

// =============================================================================
// MOCK VIDEO DECODER
// =============================================================================

/// Mock decoder for development and testing
/// Generates animated gradient frames
pub struct MockVideoDecoder {
    metadata: VideoMetadata,
    position: f64,
    playing: bool,
    frame_interval: f64,
    time_since_frame: f64,
    last_frame: Option<VideoFrame>,
}

impl MockVideoDecoder {
    pub fn new() -> Self {
        Self {
            metadata: VideoMetadata::default(),
            position: 0.0,
            playing: false,
            frame_interval: 1.0 / 30.0, // 30 FPS
            time_since_frame: 0.0,
            last_frame: None,
        }
    }
}

impl Default for MockVideoDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoDecoder for MockVideoDecoder {
    fn open(&mut self, source: &VideoSource) -> Result<VideoMetadata, VideoError> {
        match source {
            VideoSource::Mock { duration, width, height } => {
                self.metadata = VideoMetadata {
                    duration: *duration,
                    width: *width,
                    height: *height,
                    framerate: 30.0,
                    codec: "mock".to_string(),
                    has_audio: false,
                    audio_codec: None,
                };
                self.position = 0.0;
                self.playing = false;
                Ok(self.metadata.clone())
            }
            VideoSource::File(path) => {
                // Mock: pretend to open file
                self.metadata = VideoMetadata {
                    duration: 60.0,
                    width: 1920,
                    height: 1080,
                    framerate: 30.0,
                    codec: "mock-file".to_string(),
                    has_audio: true,
                    audio_codec: Some("aac".to_string()),
                };
                Ok(self.metadata.clone())
            }
            VideoSource::Url(url) => {
                // Mock: pretend to open URL
                self.metadata = VideoMetadata {
                    duration: 120.0,
                    width: 1280,
                    height: 720,
                    framerate: 24.0,
                    codec: "mock-stream".to_string(),
                    has_audio: true,
                    audio_codec: Some("aac".to_string()),
                };
                Ok(self.metadata.clone())
            }
            VideoSource::Memory { data, format } => {
                // Mock: pretend to decode from memory
                self.metadata = VideoMetadata {
                    duration: 30.0,
                    width: 640,
                    height: 480,
                    framerate: 30.0,
                    codec: format.clone(),
                    has_audio: false,
                    audio_codec: None,
                };
                Ok(self.metadata.clone())
            }
        }
    }

    fn decode_frame(&mut self) -> Result<Option<VideoFrame>, VideoError> {
        if self.position >= self.metadata.duration {
            return Err(VideoError::EndOfStream);
        }

        if self.time_since_frame >= self.frame_interval {
            self.time_since_frame = 0.0;
            
            // Generate animated gradient frame
            let frame = VideoFrame::gradient(
                self.metadata.width.min(320), // Scale down for performance
                self.metadata.height.min(180),
                self.position,
            );
            
            self.last_frame = Some(frame.clone());
            Ok(Some(frame))
        } else {
            // Return cached frame
            Ok(self.last_frame.clone())
        }
    }

    fn seek(&mut self, position: f64) -> Result<(), VideoError> {
        self.position = position.clamp(0.0, self.metadata.duration);
        self.time_since_frame = self.frame_interval; // Force new frame
        Ok(())
    }

    fn position(&self) -> f64 {
        self.position
    }

    fn duration(&self) -> f64 {
        self.metadata.duration
    }

    fn set_playing(&mut self, playing: bool) {
        self.playing = playing;
    }

    fn is_playing(&self) -> bool {
        self.playing
    }

    fn update(&mut self, dt: f32) {
        if self.playing && self.position < self.metadata.duration {
            self.position += dt as f64;
            self.time_since_frame += dt as f64;
        }
    }

    fn is_finished(&self) -> bool {
        self.position >= self.metadata.duration
    }

    fn reset(&mut self) {
        self.position = 0.0;
        self.playing = false;
        self.time_since_frame = 0.0;
        self.last_frame = None;
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_decoder_open() {
        let mut decoder = MockVideoDecoder::new();
        let source = VideoSource::Mock {
            duration: 10.0,
            width: 640,
            height: 360,
        };
        
        let metadata = decoder.open(&source).unwrap();
        assert_eq!(metadata.duration, 10.0);
        assert_eq!(metadata.width, 640);
        assert_eq!(metadata.height, 360);
    }

    #[test]
    fn test_mock_decoder_playback() {
        let mut decoder = MockVideoDecoder::new();
        decoder.open(&VideoSource::default()).unwrap();
        
        decoder.set_playing(true);
        assert!(decoder.is_playing());
        
        // Simulate some frames
        for _ in 0..10 {
            decoder.update(0.016);
        }
        
        assert!(decoder.position() > 0.0);
    }

    #[test]
    fn test_mock_decoder_seek() {
        let mut decoder = MockVideoDecoder::new();
        decoder.open(&VideoSource::Mock {
            duration: 60.0,
            width: 640,
            height: 360,
        }).unwrap();
        
        decoder.seek(30.0).unwrap();
        assert!((decoder.position() - 30.0).abs() < 0.01);
    }

    #[test]
    fn test_video_frame_gradient() {
        let frame = VideoFrame::gradient(100, 100, 0.5);
        assert_eq!(frame.width, 100);
        assert_eq!(frame.height, 100);
        assert_eq!(frame.data.len(), 100 * 100 * 4); // RGBA
    }

    #[test]
    fn test_playback_state() {
        assert!(!PlaybackState::Stopped.is_active());
        assert!(PlaybackState::Playing.is_active());
        assert!(!PlaybackState::Paused.is_active());
        assert!(PlaybackState::Buffering.is_active());
    }
}
