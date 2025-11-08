use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FrameTimer(pub Timer);

#[derive(Component)]
pub struct FallingCube;

#[point_class(
    group("spawner"), 
    iconsprite({ path: "sprites/camera.png", scale: 0.1 })
)]
#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add = spawn_camera)]
/// Spawns the world camera
pub struct Camera;

fn spawn_camera(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    world
        .commands()
        .entity(entity)
        .insert((Name::new("Main camera"), Camera3d::default()));
}

#[point_class(
    group("spawner"), 
    iconsprite({ path: "sprites/box_spawn.png", scale: 0.1 }),
)]
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[component(on_add = Self::spawn_cube_emitter)]
/// Emit boxes over time
pub struct CubeEmitter {
    /// Frequency that cubes are emitted
    #[class(default = 2.0)]
    pub spawn_rate_seconds: f32,
}

impl CubeEmitter {
    fn spawn_cube_emitter(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
        let duration = world.get::<CubeEmitter>(entity).unwrap().spawn_rate_seconds;
        info!("spawn cube emitter: {entity} {duration}sec");
        world
            .commands()
            .entity(entity)
            .insert(FrameTimer(Timer::from_seconds(
                duration,
                TimerMode::Repeating,
            )));
    }
}
