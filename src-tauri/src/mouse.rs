use std::thread;
use std::time::Duration;


use enigo::{
    Button, Coordinate,
    Direction::{ Press, Release},
    Enigo, Mouse, Settings,
};

use crate::window::set_transparency;


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct MouseClickOpts {
    pub button: MouseButton,
    pub duration: u64,
    pub interval: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct HomeClickOpts {
    pub button: MouseButton,
    pub duration: u64,
    pub click_at_start: bool,
    pub click_in_between: bool,
    pub click_at_end: bool,
}


fn handle_home_click(enigo: &mut Enigo, home: Point, home_opts: HomeClickOpts) -> Result<(), String> {
    // If want to click app window, must first set the app to transparent before using this fn

    let _ = enigo.move_mouse(home.x.round() as i32, home.y.round() as i32, Coordinate::Abs);
    // thread::sleep(Duration::from_millis(5));
    match home_opts.button {
        MouseButton::Left => {
            let _ = enigo.button(Button::Left, Press);
            thread::sleep(Duration::from_millis(home_opts.duration as u64));
            let _ = enigo.button(Button::Left, Release);
        },
        MouseButton::Middle => {
            let _ = enigo.button(Button::Middle, Press);
            thread::sleep(Duration::from_millis(home_opts.duration as u64));
            let _ = enigo.button(Button::Middle, Release);
        },
        MouseButton::Right => {
            let _ = enigo.button(Button::Right, Press);
            thread::sleep(Duration::from_millis(home_opts.duration as u64));
            let _ = enigo.button(Button::Right, Release);
        },
    }
    Ok(())
}


#[tauri::command]
pub async fn mouse_click_points(window: tauri::Window, points: Vec<Point>, home: Option<Point>, grid_options: MouseClickOpts, home_options: HomeClickOpts) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    set_transparency(window.clone(), true)?;
    
    let has_home: bool = home.is_some();

    if has_home && home_options.click_at_start {
        handle_home_click(&mut enigo, home.clone().unwrap(), home_options.clone())?;

        if points.len() > 0 {
            thread::sleep(Duration::from_millis(grid_options.interval as u64));
        }
    }
    
    // Grid Click
    for (i, point) in points.iter().enumerate() {
        let _ = enigo.move_mouse(point.x.round() as i32, point.y.round() as i32, Coordinate::Abs);
        // thread::sleep(Duration::from_millis(5));
        match grid_options.button {
            MouseButton::Left => {
                let _ = enigo.button(Button::Left, Press);
                thread::sleep(Duration::from_millis(grid_options.duration as u64));
                let _ = enigo.button(Button::Left, Release);
            },
            MouseButton::Middle => {
                let _ = enigo.button(Button::Middle, Press);
                thread::sleep(Duration::from_millis(grid_options.duration as u64));
                let _ = enigo.button(Button::Middle, Release);
            },
            MouseButton::Right => {
                let _ = enigo.button(Button::Right, Press);
                thread::sleep(Duration::from_millis(grid_options.duration as u64));
                let _ = enigo.button(Button::Right, Release);
            },
        }

        if i < points.len() - 1 {
            thread::sleep(Duration::from_millis(grid_options.interval as u64));
            if has_home && home_options.click_in_between {
                handle_home_click(&mut enigo, home.clone().unwrap(), home_options.clone())?;
                thread::sleep(Duration::from_millis(grid_options.interval as u64));            
            }
        }
    }

    if has_home && home_options.click_at_end {
        handle_home_click(&mut enigo, home.clone().unwrap(), home_options.clone())?;
    }

    set_transparency(window, false)?;
    Ok(())
}


#[tauri::command]
pub fn get_mouse_pos() -> Result<Point, String> {
    let enigo = Enigo::new(&Settings::default()).unwrap();
    let (x, y) = enigo.location().unwrap();
    let x = x as f64;
    let y = y as f64;

    Ok(Point { x, y })
}


#[tauri::command]
pub fn mouse_move_to(point: Point) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let _ = enigo.move_mouse(point.x.round() as i32, point.y.round() as i32, Coordinate::Abs);

    Ok(())
}