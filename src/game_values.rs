use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Resource, InspectorOptions, Serialize, Deserialize, Default)]
pub struct GameValues {
    pub camera_speed: f32,
    pub cube_kill_y: f32,
}
