use super::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        // Plugins
        register_trenchbroom(app);

        // Registers

        // States

        // Events

        // Observers
        app.add_observer(observers::spawn_cube);

        // Resources

        // Startups
        app.add_systems(Startup, startups::create_world);

        // Updates
        app.add_systems(PreUpdate, updates::tick_timers);
        app.add_systems(Update, updates::rotate_camera);
        app.add_systems(Update, updates::test_cube_out_of_bounds);
        app.add_systems(Update, updates::spawn_cubes);
    }
}

fn register_trenchbroom(app: &mut App) {
    use bevy_trenchbroom::prelude::*;
    use bevy_trenchbroom_avian::AvianPhysicsBackend;

    #[cfg(not(feature = "dev_native"))]
    use bevy_trenchbroom::config::WriteTrenchBroomConfigOnStartPlugin;

    let config = TrenchBroomConfig::new("trenchbroom-demo")
        .default_solid_spawn_hooks(|| SpawnHooks::new().convex_collider());

    #[cfg(feature = "dev_native")]
    app.add_plugins(TrenchBroomPlugins(config));

    #[cfg(not(feature = "dev_native"))]
    app.add_plugins(
        TrenchBroomPlugins(config)
            .build()
            .disable::<WriteTrenchBroomConfigOnStartPlugin>(),
    );

    app.add_plugins(TrenchBroomPhysicsPlugin::new(AvianPhysicsBackend));
}
