use super::*;
use avian3d::prelude::*;
use bevy::prelude::*;

pub fn spawn_cube(
    ev: On<events::SpawnCube>,
    mut commands: Commands,
    cube_emitters: Query<&Transform, With<components::CubeEmitter>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let parent = ev.entity;
    let parent_trans = cube_emitters.get(parent).unwrap();
    commands.entity(parent).with_children(|parent| {
        info!("Spawn cube");
        parent
            .spawn((
                Name::new("Bouncy cube"),
                RigidBody::Dynamic,
                Collider::cuboid(0.5, 0.5, 0.5),
                Restitution::new(0.4),
                AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
                Mesh3d(meshes.add(Cuboid::from_length(0.5))),
                MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
                *parent_trans,
                LinearVelocity::default(),
                components::FallingCube,
            ))
            .observe(despawn_cube);
    });
}

pub fn despawn_cube(ev: On<events::DespawnCube>, mut commands: Commands) {
    info!("Depsawn cube pos: {}", ev.entity);
    commands.entity(ev.entity).despawn();
}
