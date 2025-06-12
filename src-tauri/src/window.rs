use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::*;


#[tauri::command]
pub fn toggle_transparency(window: tauri::Window) -> Result<bool, String> {
    match window.hwnd() {
        Ok(hwnd_handle) => {
            let hwnd = HWND(hwnd_handle.0 as isize);
            let new_transparent_state;
            
            unsafe {
                // Get current style
                let current_style = GetWindowLongA(hwnd, GWL_EXSTYLE);
                
                // Check if WS_EX_TRANSPARENT is currently set
                let is_currently_transparent = (current_style as u32 & WS_EX_TRANSPARENT.0) != 0;
                
                // Define base styles we want to keep
                let base_style = WS_EX_APPWINDOW | WS_EX_COMPOSITED | WS_EX_LAYERED | WS_EX_TOPMOST;
                
                // Toggle transparent flag
                new_transparent_state = !is_currently_transparent;
                let new_style = if is_currently_transparent {
                    base_style // Remove WS_EX_TRANSPARENT
                } else {
                    base_style | WS_EX_TRANSPARENT // Add WS_EX_TRANSPARENT
                };
                
                // Apply the style
                SetWindowLongA(hwnd, GWL_EXSTYLE, new_style.0 as i32);
            };
            
            Ok(new_transparent_state)
        }
        Err(e) => Err(format!("Failed to get window handle: {}", e)),
    }
}

#[tauri::command]
pub fn set_transparency(window: tauri::Window, is_transparent: bool) -> Result<(), String> {
    match window.hwnd() {
        Ok(hwnd_handle) => {
            let hwnd = HWND(hwnd_handle.0 as isize);
            unsafe {
                // Define base styles we want to keep
                let base_style = WS_EX_APPWINDOW | WS_EX_COMPOSITED | WS_EX_LAYERED | WS_EX_TOPMOST;
                
                // Add transparent flag if requested
                let new_style = if is_transparent {
                    base_style | WS_EX_TRANSPARENT
                } else {
                    base_style
                };
                
                // Apply the style
                SetWindowLongA(hwnd, GWL_EXSTYLE, new_style.0 as i32);
            };
            Ok(())
        },
        Err(e) => Err(format!("Failed to get window handle: {}", e)),
    }
}