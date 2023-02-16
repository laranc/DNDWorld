use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{
    characters::CHARACTER_SCALE,
    components::{
        CharacterComponent, CharactersComponent, CustomMapComponent, CustomSpriteComponent,
        MapComponent, MapFace, MapsComponent,
    },
    resources::{
        CurrentCustomMap, CursorPosition, CustomMapList, CustomSpriteList, DraggingSprite,
    },
    GameState,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition::default())
            .add_system(zoom_system)
            .add_system(move_system)
            .add_system(cursor_system)
            .add_system(hover_system);
    }
}

fn cursor_system(
    windows: Res<Windows>,
    mut e_cursor: EventReader<CursorMoved>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut cursor_pos: ResMut<CursorPosition>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = windows.get_primary().unwrap();
    for e in e_cursor.iter() {
        let screen_pos = e.position;
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (screen_pos / window_size) * 2. - Vec2::ONE;
        let world_ndc = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = world_ndc.project_point3(ndc.extend(-1.));
        let world_pos: Vec2 = world_pos.truncate();
        cursor_pos.0 = world_pos;
    }
}

fn zoom_system(
    q_maps: Query<&Children, (With<MapsComponent>, Without<CharactersComponent>)>,
    mut q_map: Query<(&mut Transform, &MapComponent), Without<CharacterComponent>>,
    mut q_custom_maps: Query<(&mut Transform, &CustomMapComponent), Without<MapComponent>>,
    custom_map_list: Res<CustomMapList>,
    current_custom_map: Res<CurrentCustomMap>,
    mut e_scroll: EventReader<MouseWheel>,
    state: Res<State<GameState>>,
) {
    match state.current() {
        GameState::CustomMap => {
            if custom_map_list.1 > 0 {
                for (mut transform, custom_map_component) in q_custom_maps.iter_mut() {
                    if transform.scale.x <= 0. && transform.scale.y <= 0. {
                        transform.scale.x = 1.;
                        transform.scale.y = 1.;
                    }
                    if !custom_map_component.map.3
                        && Some(custom_map_component.map.clone()) == current_custom_map.0
                    {
                        for e in e_scroll.iter() {
                            match e.unit {
                                MouseScrollUnit::Line => {
                                    if transform.scale.x >= 0. && transform.scale.y >= 0. {
                                        transform.scale.x += e.y;
                                        transform.scale.y += e.y;
                                    }
                                }
                                MouseScrollUnit::Pixel => {
                                    if transform.scale.x >= 0. && transform.scale.y >= 0. {
                                        transform.scale.x += e.y;
                                        transform.scale.y += e.y;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            let maps = q_maps.single();
            for &map in maps.iter() {
                let (mut transform, map_component) = q_map.get_mut(map).unwrap();
                if transform.scale.x <= 0. && transform.scale.y <= 0. {
                    transform.scale.x = 1.;
                    transform.scale.y = 1.;
                }
                let map_face: MapFace;
                match state.current() {
                    GameState::WorldMap => map_face = MapFace::World,
                    GameState::TownMap => map_face = MapFace::Town,
                    _ => map_face = MapFace::World,
                }
                for e in e_scroll.iter() {
                    match e.unit {
                        MouseScrollUnit::Line => {
                            if map_component.face == map_face {
                                if transform.scale.x >= 0. && transform.scale.y >= 0. {
                                    transform.scale.x += e.y;
                                    transform.scale.y += e.y;
                                }
                            }
                        }
                        MouseScrollUnit::Pixel => {
                            if map_component.face == map_face {
                                if transform.scale.x >= 0. && transform.scale.y >= 0. {
                                    transform.scale.x += e.y;
                                    transform.scale.y += e.y;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn move_system(
    q_camera: Query<&GlobalTransform, With<Camera>>,
    i_buttons: Res<Input<MouseButton>>,
    cursor_pos: Res<CursorPosition>,
) {
    let camera_transform = q_camera.single();
    let prev_cursor_pos = cursor_pos.0;
    if i_buttons.pressed(MouseButton::Middle) {
        (
            camera_transform.translation().x,
            camera_transform.translation().y,
        ) = (
            camera_transform.translation().x + (cursor_pos.0.x - prev_cursor_pos.x),
            camera_transform.translation().y + (cursor_pos.0.y - prev_cursor_pos.y),
        );
    }
}

fn hover_system(
    cursor_pos: Res<CursorPosition>,
    q_characters: Query<&Children, (With<CharactersComponent>, Without<MapsComponent>)>,
    mut q_character: Query<(&Transform, &mut CharacterComponent), Without<MapComponent>>,
    mut q_custom_sprites: Query<(&Transform, &Handle<Image>, &mut CustomSpriteComponent)>,
    mut dragging_sprite: ResMut<DraggingSprite>,
    custom_sprite_list: Res<CustomSpriteList>,
    images: Res<Assets<Image>>,
) {
    let characters = q_characters.single();
    for &character in characters.iter() {
        let (transform, mut character_component) = q_character.get_mut(character).unwrap();
        let half_width = Vec2::splat(CHARACTER_SCALE).x / 2.;
        let half_height = Vec2::splat(CHARACTER_SCALE).y / 2.;
        if transform.translation.x - half_width < cursor_pos.0.x
            && transform.translation.x + half_width > cursor_pos.0.x
            && transform.translation.y - half_height < cursor_pos.0.y
            && transform.translation.y + half_height > cursor_pos.0.y
        {
            character_component.is_hovered = true;
            dragging_sprite.0 = false;
        } else {
            character_component.is_hovered = false;
        }
    }
    if custom_sprite_list.1 > 0 {
        for (transform, image_handle, mut custom_sprite_component) in q_custom_sprites.iter_mut() {
            let image = images.get(image_handle).unwrap();
            let half_width = image.size().x / 2.;
            let half_height = image.size().y / 2.;
            if transform.translation.x - half_width < cursor_pos.0.x
                && transform.translation.x + half_width > cursor_pos.0.x
                && transform.translation.y - half_height < cursor_pos.0.y
                && transform.translation.y + half_height > cursor_pos.0.y
            {
                custom_sprite_component.is_hovered = true;
                dragging_sprite.0 = false;
            } else {
                custom_sprite_component.is_hovered = false;
            }
        }
    }
}
