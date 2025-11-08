use super::*;
use crate::game_parameters::GameParameters;
use bevy::prelude::*;

pub fn rotate_camera(
    time: Res<Time>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
    params: Res<GameParameters>,
) {
    let delta = time.delta_secs();
    let cam_speed = params.values().camera_speed;

    camera.rotate_around(Vec3::ZERO, Quat::from_rotation_y(cam_speed * delta));
    camera.look_at(Vec3::ZERO, Vec3::Y);
}

pub fn test_cube_out_of_bounds(
    mut commands: Commands,
    params: Res<GameParameters>,
    q_cube: Query<(&Transform, Entity), With<components::FallingCube>>,
) {
    for (trans, entity) in q_cube {
        if trans.translation.y < params.values().cube_kill_y {
            commands.trigger(events::DespawnCube { entity });
        }
    }
}

pub fn tick_timers(time: Res<Time>, mut query: Query<&mut components::FrameTimer>) {
    for mut my_timer in &mut query {
        my_timer.0.tick(time.delta());
    }
}

pub fn spawn_cubes(
    mut commands: Commands,
    q_cube_spawners: Query<(Entity, &components::FrameTimer)>,
) {
    for (entity, timer) in q_cube_spawners {
        if timer.0.is_finished() {
            commands.trigger(events::SpawnCube { entity });
        }
    }
}
