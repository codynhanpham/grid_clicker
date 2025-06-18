use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::*;


#[tauri::command]
pub fn toggle_transparency(window: tauri::Window) -> Result<bool, String> {
    match window.hwnd() {
        Ok(hwnd_handle) => {
            let hwnd = HWND(hwnd_handle.0);
            let new_transparent_state;
            
            unsafe {
                // Get current style
                let current_style = GetWindowLongA(hwnd, GWL_EXSTYLE);
                let base_style = WS_EX_APPWINDOW | WS_EX_COMPOSITED | WS_EX_LAYERED | WS_EX_TOPMOST;
                
                // Check if WS_EX_TRANSPARENT is currently set
                let is_currently_transparent = (current_style as u32 & WS_EX_TRANSPARENT.0) != 0;
                
                // Toggle transparent flag
                new_transparent_state = !is_currently_transparent;
                let new_style = if is_currently_transparent {
                    base_style // Remove WS_EX_TRANSPARENT = revert to base style
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
            let hwnd = HWND(hwnd_handle.0);
            unsafe {
                // Default window style
                let base_style = WS_EX_APPWINDOW | WS_EX_COMPOSITED | WS_EX_LAYERED | WS_EX_TOPMOST;
                
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