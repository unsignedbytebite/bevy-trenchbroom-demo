use bevy::{
    prelude::*,
    window::{CursorOptions, WindowResolution},
};
use serde::Deserialize;
use std::io;

#[derive(Deserialize)]
struct WindowParameters {
    resolution: (u16, u16),
    title: String,
    app_name: String,
    resizable: bool,
    mouse_visible: bool,
    windowed: bool,
}

impl Default for WindowParameters {
    fn default() -> Self {
        Self {
            resolution: (1920 / 2, 1080 / 2),
            title: "my game".to_string(),
            app_name: "my_game".to_string(),
            resizable: true,
            mouse_visible: true,
            windowed: true,
        }
    }
}

#[cfg(feature = "dev_native")]
fn get_window_parameter_file() -> Result<String, io::Error> {
    std::fs::read_to_string("./assets/window_parameters.ron")
}

#[cfg(not(feature = "dev_native"))]
fn get_window_parameter_file() -> Result<String, io::Error> {
    Ok(include_str!("../assets/window_parameters.ron").to_string())
}

pub fn windows_settings() -> WindowPlugin {
    let window_parameters = match get_window_parameter_file() {
        Ok(content) => match ron::from_str::<WindowParameters>(&content) {
            Ok(params) => params,
            Err(err) => {
                error!("Error loading ./assets/window_parameters.ron: {err:?}");
                WindowParameters::default()
            }
        },
        Err(err) => {
            error!("Error loading window parameters: {err:?}");
            WindowParameters::default()
        }
    };

    let title = window_parameters.title;
    let name = Some(window_parameters.app_name);
    let (resolution_x, resolution_y) = window_parameters.resolution;
    let mut resizable = window_parameters.resizable;
    let mouse_visible = window_parameters.mouse_visible;
    let mode = if window_parameters.windowed {
        bevy::window::WindowMode::Windowed
    } else {
        resizable = true;
        bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
    };

    WindowPlugin {
        primary_window: Some(Window {
            title,
            name,
            resolution: WindowResolution::new(resolution_x as u32, resolution_y as u32),
            resizable,
            mode,
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..Default::default()
        }),
        primary_cursor_options: Some(CursorOptions {
            visible: mouse_visible,
            ..Default::default()
        }),
        ..Default::default()
    }
}
