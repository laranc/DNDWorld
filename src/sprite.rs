use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{
    check_file,
    components::CustomSpriteComponent,
    resources::{CursorPosition, CustomSprite, CustomSpriteList, DraggingSprite, ErrorLabel},
    GameState,
};

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CustomSprite::default())
            .insert_resource(CustomSpriteList::default())
            .insert_resource(ErrorLabel::default())
            .add_system_set(
                SystemSet::on_update(GameState::TownMap)
                    .with_system(add_custom_sprite_system)
                    .with_system(custom_sprite_list_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::WorldMap)
                    .with_system(add_custom_sprite_system)
                    .with_system(custom_sprite_list_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::CustomMap)
                    .with_system(add_custom_sprite_system)
                    .with_system(custom_sprite_list_system),
            )
            .add_system(custom_sprite_system);
    }
}

fn add_custom_sprite_system(
    commands: Commands,
    assets: Res<AssetServer>,
    mut egui_ctx: ResMut<EguiContext>,
    mut custom_sprite: ResMut<CustomSprite>,
    mut custom_sprite_list: ResMut<CustomSpriteList>,
    mut error_label: ResMut<ErrorLabel>,
) {
    egui::Window::new("Spawn Custom Sprites").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("File name e.g. sprite.png (paste png file in assets/custom/ directory)");
        ui.text_edit_singleline(&mut custom_sprite.0);
        ui.label("Sprite name");
        ui.text_edit_singleline(&mut custom_sprite.1);
        ui.add(egui::Slider::new(&mut custom_sprite.2, 0.1..=10.).text("Sprite Scale"));
        if ui.button("Spawn Sprite").clicked() {
            match check_file(&custom_sprite.0) {
                Some(s) => {
                    custom_sprite.0 = s;
                    spawn_custom_sprite(
                        commands,
                        assets,
                        &custom_sprite.0,
                        &custom_sprite.1,
                        custom_sprite.2,
                    );
                    custom_sprite_list.0.push(custom_sprite.1.clone());
                    custom_sprite_list.1 = custom_sprite_list.0.len();
                    error_label.update("".to_owned());
                }
                None => {
                    error_label.update("Failed to find file".to_owned());
                }
            }
        }
        error_label.content(ui);
    });
}

fn custom_sprite_list_system(
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    mut custom_sprite_list: ResMut<CustomSpriteList>,
    q_custom_sprites: Query<(Entity, &CustomSpriteComponent)>,
) {
    if custom_sprite_list.1 > 0 {
        egui::SidePanel::right("custom_sprite_list").show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.heading("Custom Sprites");
                if ui.button("Delete all").clicked() {
                    for (e, _) in q_custom_sprites.iter() {
                        commands.entity(e).despawn();
                    }
                    custom_sprite_list.0.clear();
                    custom_sprite_list.1 = 0;
                }
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    for i in 0..custom_sprite_list.0.len() {
                        ui.label(&custom_sprite_list.0[i]);
                        if ui.button("Delete").clicked() {
                            for (e, csc) in q_custom_sprites.iter() {
                                if csc.name == custom_sprite_list.0[i] {
                                    commands.entity(e).despawn();
                                    custom_sprite_list.0.remove(i);
                                    custom_sprite_list.1 = custom_sprite_list.0.len();
                                }
                            }
                        }
                    }
                });
            });
        });
    }
}

fn custom_sprite_system(
    mut q_custom_sprites: Query<(&mut Transform, &CustomSpriteComponent)>,
    cursor_pos: Res<CursorPosition>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut dragging_sprite: ResMut<DraggingSprite>,
) {
    for (mut transform, custom_sprite_component) in q_custom_sprites.iter_mut() {
        if mouse_buttons.pressed(MouseButton::Left) {
            if custom_sprite_component.is_hovered && !dragging_sprite.0 {
                dragging_sprite.0 = true;
                transform.translation.x = cursor_pos.0.x;
                transform.translation.y = cursor_pos.0.y;
            }
        }
    }
}

pub fn spawn_custom_sprite(
    mut commands: Commands,
    assets: Res<AssetServer>,
    sprite_file: &String,
    sprite_name: &String,
    sprite_scale: f32,
) -> () {
    commands
        .spawn(SpriteBundle {
            texture: assets.load(&format!("custom/{}", sprite_file)),
            ..Default::default()
        })
        .insert(Name::new(sprite_name.clone()))
        .insert(CustomSpriteComponent {
            name: sprite_name.clone(),
            ..Default::default()
        })
        .insert(SpatialBundle {
            visibility: Visibility { is_visible: true },
            transform: Transform {
                translation: Vec3::new(0., 0., 200.),
                scale: Vec3::new(sprite_scale, sprite_scale, sprite_scale),
                ..Default::default()
            },
            ..Default::default()
        });
}
