use avian3d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::camera::visibility::RenderLayers;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InspectorState {
    #[default]
    Hidden,
    Visible,
}

pub fn register(app: &mut App) {
    let fps_overlay = FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextFont {
                font_size: 18.0,
                ..Default::default()
            },
            text_color: Color::WHITE,
            enabled: false,
            frame_time_graph_config: FrameTimeGraphConfig {
                enabled: false,
                min_fps: 24.0,
                target_fps: 60.0,
            },
            ..Default::default()
        },
    };

    app.add_plugins((
        fps_overlay,
        EguiPlugin::default(),
        ResourceInspectorPlugin::<super::game_parameters::GameParameters>::default()
            .run_if(in_state(InspectorState::Visible)),
        WorldInspectorPlugin::default().run_if(in_state(InspectorState::Visible)),
        PhysicsDebugPlugin,
    ))
    .insert_gizmo_config(
        PhysicsGizmos::default(),
        GizmoConfig {
            enabled: false,
            ..Default::default()
        },
    );

    app.init_state::<InspectorState>();

    app.add_systems(PreStartup, create_egui_cam);
    app.add_systems(Update, toggle);

    info!("Press 'F1' to toggle dev mode");
}

/// Create the egui camera
fn create_egui_cam(mut commands: Commands) {
    commands.spawn((
        Name::new("egui_render_camera"),
        Camera2d,
        RenderLayers::layer(16),
        Camera {
            order: 100,
            ..Default::default()
        },
    ));
}

/// Toggle the dev mode
fn toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<InspectorState>>,
    mut next_state: ResMut<NextState<InspectorState>>,
    mut fps_overlay: ResMut<FpsOverlayConfig>,
    mut gizmo_store: ResMut<GizmoConfigStore>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        match state.get() {
            InspectorState::Hidden => {
                fps_overlay.enabled = true;
                fps_overlay.frame_time_graph_config.enabled = true;
                next_state.set(InspectorState::Visible);
                gizmo_store.config_mut::<PhysicsGizmos>().0.enabled = true;
            }
            InspectorState::Visible => {
                fps_overlay.enabled = false;
                fps_overlay.frame_time_graph_config.enabled = false;
                next_state.set(InspectorState::Hidden);
                gizmo_store.config_mut::<PhysicsGizmos>().0.enabled = false;
            }
        }

        info!("Dev mode set: {}", fps_overlay.enabled);
    }
}
