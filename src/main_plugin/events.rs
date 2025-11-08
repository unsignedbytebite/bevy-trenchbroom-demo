use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct SpawnCube {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct DespawnCube {
    pub entity: Entity,
}
