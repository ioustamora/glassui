//! GlassUI Clipboard Support
//!
//! Cross-platform clipboard integration using arboard.

use std::sync::Mutex;

// Global clipboard instance (lazy initialized)
static CLIPBOARD: Mutex<Option<arboard::Clipboard>> = Mutex::new(None);

/// Get or initialize the clipboard
fn get_clipboard() -> Result<std::sync::MutexGuard<'static, Option<arboard::Clipboard>>, String> {
    let mut guard = CLIPBOARD.lock().map_err(|e| format!("Clipboard lock failed: {}", e))?;
    
    if guard.is_none() {
        match arboard::Clipboard::new() {
            Ok(cb) => *guard = Some(cb),
            Err(e) => return Err(format!("Failed to initialize clipboard: {}", e)),
        }
    }
    
    Ok(guard)
}

/// Copy text to clipboard
pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut guard = get_clipboard()?;
    if let Some(cb) = guard.as_mut() {
        cb.set_text(text).map_err(|e| format!("Copy failed: {}", e))?;
        log::debug!("Copied to clipboard: {} chars", text.len());
        Ok(())
    } else {
        Err("Clipboard not available".to_string())
    }
}

/// Paste text from clipboard
pub fn paste_from_clipboard() -> Result<String, String> {
    let mut guard = get_clipboard()?;
    if let Some(cb) = guard.as_mut() {
        let text = cb.get_text().map_err(|e| format!("Paste failed: {}", e))?;
        log::debug!("Pasted from clipboard: {} chars", text.len());
        Ok(text)
    } else {
        Err("Clipboard not available".to_string())
    }
}

/// Clear the clipboard
pub fn clear_clipboard() -> Result<(), String> {
    let mut guard = get_clipboard()?;
    if let Some(cb) = guard.as_mut() {
        cb.clear().map_err(|e| format!("Clear failed: {}", e))?;
        Ok(())
    } else {
        Err("Clipboard not available".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_clipboard_roundtrip() {
        let test_text = "Hello, GlassUI!";
        
        // Note: This test may fail in CI environments without display
        if copy_to_clipboard(test_text).is_ok() {
            let result = paste_from_clipboard();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), test_text);
        }
    }
}
