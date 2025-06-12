use tauri::{Manager, WebviewWindow};
use tauri_plugin_window_state::{StateFlags, WindowExt};

mod mouse;
mod window;


#[cfg(desktop)]
pub fn run() {
    let mut builder =
        tauri::Builder::default().plugin(tauri_plugin_global_shortcut::Builder::new().build());


    builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
        let _ = app
            .get_webview_window("main")
            .expect("no main window")
            .set_focus();
    }));

    builder
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                // Debug-mode setup
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
                
                let window: WebviewWindow = app.get_webview_window("main").unwrap();
                window.restore_state(StateFlags::all()).unwrap();
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            exit_app,
            window::toggle_transparency,
            mouse::mouse_click_points,
            mouse::get_mouse_pos,
            mouse::mouse_move_to,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn exit_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0i32);
}