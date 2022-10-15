use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_egui::{egui, EguiContext, EguiPlugin};

mod camera;
mod characters;
mod components;
mod debug;
mod draw;
mod map;
mod resources;

use camera::CameraPlugin;
use characters::CharactersPlugin;
use debug::DebugPlugin;
use draw::DrawPlugin;
use map::MapPlugin;
use resources::GameState;

pub const RESOLUTION: f32 = 16. / 9.;
pub const SCREEN_HEIGHT: f32 = 900.;

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

impl Default for WinSize {
    fn default() -> Self {
        Self {
            w: SCREEN_HEIGHT * RESOLUTION,
            h: SCREEN_HEIGHT,
        }
    }
}

fn main() {
    App::new()
        .add_state(GameState::WorldMap)
        .insert_resource(WindowDescriptor {
            title: "DND World".to_string(),
            width: WinSize::default().w,
            height: WinSize::default().h,
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(CharactersPlugin)
        .add_plugin(DrawPlugin)
        .add_plugin(CameraPlugin)
        .add_startup_system(setup_system)
        .add_system(control_panel_system)
        .run();
}

fn setup_system(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Name::new("Camera"));

    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);
}

fn control_panel_system(mut egui_ctx: ResMut<EguiContext>, mut state: ResMut<State<GameState>>) {
    egui::TopBottomPanel::bottom("Control Panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| match state.current() {
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
            }
        });
    });
}
