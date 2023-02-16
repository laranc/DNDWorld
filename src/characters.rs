use bevy::prelude::*;

use crate::{
    components::{CharacterComponent, CharactersComponent, MapComponent, MapsComponent},
    resources::{CursorPosition, DraggingSprite, LoadedFromFile, SpriteSheet},
};
pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DraggingSprite::default())
            .add_startup_system_to_stage(StartupStage::Startup, character_setup_system)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_characters_system)
            .add_system(characters_system);
    }
}

pub const CHARACTER_NUM: u8 = 4;
pub const CHARACTER_SCALE: f32 = 100.;

fn character_setup_system(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    loaded: Res<LoadedFromFile>,
) {
    if !loaded.0 {
        let atlas = TextureAtlas::from_grid(
            assets.load("characters/characters.png"),
            Vec2::new(11., 15.),
            4,
            1,
            Some(Vec2::default()),
            Some(Vec2::default()),
        );

        let handle = texture_atlas.add(atlas);

        commands.insert_resource(SpriteSheet(handle));
    }
}

fn spawn_characters_system(
    mut commands: Commands,
    texture_atlas: Res<SpriteSheet>,
    loaded: Res<LoadedFromFile>,
) {
    if !loaded.0 {
        let mut characters = Vec::new();
        for i in 0..=CHARACTER_NUM - 1 {
            let mut sprite = TextureAtlasSprite::new(i as usize);
            sprite.custom_size = Some(Vec2::splat(CHARACTER_SCALE));

            let character = commands
                .spawn(SpriteSheetBundle {
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
                .insert(CharacterComponent::default())
                .id();
            characters.push(character);
        }
        commands
            .spawn_empty()
            .insert(Name::new("Characters"))
            .insert(CharactersComponent)
            .insert(SpatialBundle {
                visibility: Visibility { is_visible: true },
                ..Default::default()
            })
            .push_children(&characters);
    }
}

fn characters_system(
    q_characters: Query<&Children, (With<CharactersComponent>, Without<MapsComponent>)>,
    mut q_character: Query<(&mut Transform, &mut CharacterComponent), Without<MapComponent>>,
    cursor_pos: Res<CursorPosition>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut dragging_sprite: ResMut<DraggingSprite>,
) {
    let characters = q_characters.single();
    for &character in characters.iter() {
        let (mut transform, character_component) = q_character.get_mut(character).unwrap();
        if mouse_buttons.pressed(MouseButton::Left) {
            if character_component.is_hovered && !dragging_sprite.0 {
                dragging_sprite.0 = true;
                transform.translation.x = cursor_pos.0.x;
                transform.translation.y = cursor_pos.0.y;
            }
        }
    }
}
