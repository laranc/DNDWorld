use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use std::path::Path;

mod camera;
mod characters;
mod components;
mod debug;
mod draw;
mod map;
mod resources;
mod save;
mod sprite;

use camera::CameraPlugin;
use characters::CharactersPlugin;
use debug::DebugPlugin;
use draw::DrawPlugin;
use map::MapPlugin;
use resources::{GameState, LoadedFromFile, WinSize};
use save::{SaveExitEvent, SavePlugin};
use sprite::SpritePlugin;

pub const RESOLUTION: f32 = 16. / 9.;
pub const SCREEN_HEIGHT: f32 = 900.;

fn main() {
    App::new()
        .add_state(GameState::WorldMap)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "DND World".to_string(),
                        width: WinSize::default().w,
                        height: WinSize::default().h,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                }),
        )
        .add_plugin(EguiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(SavePlugin)
        .add_plugin(MapPlugin)
        .add_plugin(CharactersPlugin)
        .add_plugin(DrawPlugin)
        .add_plugin(SpritePlugin)
        .add_plugin(CameraPlugin)
        .add_startup_system(setup_system)
        .add_system(control_panel_system)
        .run();
}

fn setup_system(mut commands: Commands, mut windows: ResMut<Windows>, loaded: Res<LoadedFromFile>) {
    if !loaded.0 {
        commands
            .spawn(Camera2dBundle::default())
            .insert(Name::new("Camera"));

        let window = windows.get_primary_mut().unwrap();
        let win_size = WinSize {
            w: window.width(),
            h: window.height(),
        };
        commands.insert_resource(win_size);
    }
}

fn control_panel_system(
    mut egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    mut ev_save_exit: EventWriter<SaveExitEvent>,
) {
    egui::TopBottomPanel::bottom("control_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            match state.current() {
                GameState::WorldMap => {
                    if ui.button("Town").clicked() {
                        state
                            .set(GameState::TownMap)
                            .expect("Failed to change state to Town Map mode")
                    }
                    if ui.button("Interior").clicked() {
                        state
                            .set(GameState::Interior)
                            .expect("Failed to change state to Interior mode");
                    }
                    if ui.button("Custom Maps").clicked() {
                        state
                            .set(GameState::CustomMap)
                            .expect("Failed to change state to Custom Map mode");
                    }
                }
                GameState::TownMap => {
                    if ui.button("World").clicked() {
                        state
                            .set(GameState::WorldMap)
                            .expect("Failed to change state to World Map mode")
                    }
                    if ui.button("Interior").clicked() {
                        state
                            .set(GameState::Interior)
                            .expect("Failed to change state to Interior mode");
                    }
                    if ui.button("Custom Maps").clicked() {
                        state
                            .set(GameState::CustomMap)
                            .expect("Failed to change state to Custom Map mode");
                    }
                }
                GameState::Interior => {
                    if ui.button("World").clicked() {
                        state
                            .set(GameState::WorldMap)
                            .expect("Failed to change state to World Map mode");
                    }
                    if ui.button("Town").clicked() {
                        state
                            .set(GameState::TownMap)
                            .expect("Failed to change state to Town Map Mode");
                    }
                    if ui.button("Custom Map").clicked() {
                        state
                            .set(GameState::CustomMap)
                            .expect("Failed to change state to Custom Map mode");
                    }
                }
                GameState::CustomMap => {
                    if ui.button("World").clicked() {
                        state
                            .set(GameState::WorldMap)
                            .expect("Failed to change state to World Map mode");
                    }
                    if ui.button("Town").clicked() {
                        state
                            .set(GameState::TownMap)
                            .expect("Failed to change state to Town Map mode");
                    }
                    if ui.button("Interior").clicked() {
                        state
                            .set(GameState::Interior)
                            .expect("Failed to change state to Interior mode");
                    }
                }
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                if ui.button("Save and Exit").clicked() {
                    ev_save_exit.send(SaveExitEvent);
                }
            });
        });
    });
}

pub fn check_file(file_name: &String) -> Option<String> {
    let mut filetype_png = false;
    let mut missing_file_ext = true;

    for s in file_name.split('.') {
        if s == "png" {
            filetype_png = true;
        }
    }
    if filetype_png {
        if Path::new(&format!("assets/custom/{}", file_name)).exists() {
            return Some(file_name.clone());
        } else {
            return None;
        }
    } else {
        for c in file_name.chars() {
            if c == '.' {
                missing_file_ext = false;
            }
        }
        if !missing_file_ext {
            if Path::new(&format!("assets/custom/{}", file_name)).exists() {
                return Some(format!("{}", file_name));
            } else {
                return None;
            }
        } else {
            if Path::new(&format!("assets/custom/{}.png", file_name)).exists() {
                return Some(format!("{}.png", file_name));
            } else {
                return None;
            }
        }
    }
}
