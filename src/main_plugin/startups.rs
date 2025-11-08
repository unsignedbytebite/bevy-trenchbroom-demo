use bevy::prelude::*;

pub fn create_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneRoot(asset_server.load("map1.map#Scene")));
}
