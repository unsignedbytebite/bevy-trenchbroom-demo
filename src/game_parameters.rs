use super::game_values::GameValues;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct GameParameters {
    values: GameValues,
}

impl GameParameters {
    pub fn values(&self) -> &GameValues {
        &self.values
    }
}

impl Default for GameParameters {
    #[cfg(feature = "dev_native")]
    fn default() -> Self {
        match std::fs::read_to_string("./assets/game_parameters.ron") {
            Ok(content) => match ron::from_str::<GameValues>(&content) {
                Ok(values) => Self { values },
                Err(err) => {
                    error!("Error loading ./assets/game_parameters.ron: {err:?}");
                    Self {
                        values: GameValues::default(),
                    }
                }
            },
            Err(err) => {
                error!("Error loading game parameters: {err:?}");

                Self {
                    values: GameValues::default(),
                }
            }
        }
    }

    #[cfg(not(feature = "dev_native"))]
    fn default() -> Self {
        let content = include_str!("../assets/game_parameters.ron");

        match ron::from_str::<GameValues>(content) {
            Ok(values) => Self { values },
            Err(err) => {
                error!("Error loading ./assets/game_parameters.ron: {err:?}");
                Self {
                    values: GameValues::default(),
                }
            }
        }
    }
}

#[cfg(feature = "dev_native")]
pub fn register(app: &mut App) {
    app.init_resource::<GameParameters>();
    app.add_systems(Update, save_file);
    info!("Press 'ctrl+s' to save game parameters");
}

#[cfg(not(feature = "dev_native"))]
pub fn register(app: &mut App) {
    app.init_resource::<GameParameters>();
}

#[cfg(feature = "dev_native")]
fn save_file(keyboard: Res<ButtonInput<KeyCode>>, game_parameters: Res<GameParameters>) {
    if keyboard.all_pressed([KeyCode::ControlLeft, KeyCode::KeyS]) {
        info!("Save game parameters");
        match ron::to_string(game_parameters.values()) {
            Ok(s) => {
                if std::fs::write("./assets/game_parameters.ron", &s)
                    .err()
                    .is_some()
                {
                    error!("Cannot save ./assets/game_parameters.ron");
                }
            }
            Err(err) => error!("Cannot save ./assets/game_parameters.ron : {err:?}"),
        }
    }
}
