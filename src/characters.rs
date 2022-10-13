use bevy::prelude::*;

use crate::{components::CharacterComponent, resources::SpriteSheet};
pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) -> () {
        app.add_startup_system_to_stage(StartupStage::Startup, character_setup_system)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_characters_system);
    }
}

pub const CHARACTER_NUM: u8 = 4;
pub const CHARACTER_SCALE: f32 = 100.;

fn character_setup_system(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    let atlas = TextureAtlas::from_grid(
        assets.load("characters/characters.png"),
        Vec2::new(11., 15.),
        4,
        1,
    );

    let handle = texture_atlas.add(atlas);

    commands.insert_resource(SpriteSheet(handle));
}

fn spawn_characters_system(mut commands: Commands, texture_atlas: Res<SpriteSheet>) {
    for i in 0..=CHARACTER_NUM - 1 {
        let mut sprite = TextureAtlasSprite::new(i as usize);
        sprite.custom_size = Some(Vec2::splat(CHARACTER_SCALE));

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite,
                texture_atlas: texture_atlas.0.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        i as f32 * CHARACTER_SCALE,
                        -(i as f32) * CHARACTER_SCALE,
                        100.,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new(format!("Character {}", i + 1)))
            .insert(CharacterComponent);
    }
}
