//#![allow(warnings)]

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

#[cfg(feature = "dev_native")]
mod dev_utils;
mod game_parameters;
mod game_values;
mod main_plugin;
mod window_parameters;

const CLEAR_COLOUR: Color = Color::hsv(219.0, 0.58, 0.93);

fn main() {
    let mut app = App::new();

    let default_plugins = DefaultPlugins
        .set(window_parameters::windows_settings())
        .set(AssetPlugin {
            #[cfg(feature = "dev_native")]
            watch_for_changes_override: Some(true),
            ..default()
        });

    app.add_plugins((
        default_plugins,
        // bevy_skein::SkeinPlugin::default(),
        avian3d::PhysicsPlugins::default(),
        main_plugin::plugin::MainPlugin,
    ));

    #[cfg(feature = "dev_native")]
    dev_utils::register(&mut app);

    app.add_systems(
        Update,
        quick_exit.run_if(input_just_pressed(KeyCode::Escape)),
    );

    app.insert_resource(ClearColor(CLEAR_COLOUR));

    game_parameters::register(&mut app);

    app.run();
}

/// A quick exit
fn quick_exit(mut ev_exit: MessageWriter<AppExit>) {
    ev_exit.write(AppExit::Success);
}

// I love baba. I love Ada i will never forget you. I will always remember you. me to when are
// you tgoing to do the cube thiningy. now, if you let me use the keyboard, thank.OK OK OK OK
// OK OK OJK OK . <3  YOU CAN YOOSE THE KEYBOAD.
