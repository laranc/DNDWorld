use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{
    characters::CHARACTER_SCALE,
    components::{CharacterComponent, CharactersComponent, MapComponent, MapFace, MapsComponent},
    resources::{CursorPosition, DraggingSprite},
    GameState,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition::default())
            .add_system(camera_system)
            .add_system(cursor_system);
    }
}

fn camera_system(
    q_maps: Query<&Children, (With<MapsComponent>, Without<CharactersComponent>)>,
    mut q_map: Query<(&mut Transform, &MapComponent), Without<CharacterComponent>>,
    mut e_scroll: EventReader<MouseWheel>,
    state: Res<State<GameState>>,
) {
    let maps = q_maps.single();
    for &map in maps.iter() {
        let (mut transform, map_component) = q_map.get_mut(map).unwrap();
        if transform.scale.x <= 0. && transform.scale.y <= 0. {
            transform.scale.x = 1.;
            transform.scale.y = 1.;
        }
        let mut map_face = MapFace::World;
        match state.current() {
            GameState::WorldMap => map_face = MapFace::World,
            GameState::TownMap => map_face = MapFace::Town,
            _ => (),
        }
        for e in e_scroll.iter() {
            match e.unit {
                MouseScrollUnit::Line => {
                    if map_component.face == map_face {
                        if transform.scale.x > 0. && transform.scale.y > 0. {
                            transform.scale.x += e.y;
                            transform.scale.y += e.y;
                        }
                    }
                }
                MouseScrollUnit::Pixel => {
                    if map_component.face == map_face {
                        transform.scale.x += e.y * 10.;
                        transform.scale.y += e.y * 10.;
                    }
                }
            }
        }
    }
}

fn cursor_system(
    windows: Res<Windows>,
    mut e_cursor: EventReader<CursorMoved>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut cursor_pos: ResMut<CursorPosition>,
    q_characters: Query<&Children, (With<CharactersComponent>, Without<MapsComponent>)>,
    mut q_character: Query<(&Transform, &mut CharacterComponent), Without<MapComponent>>,
    mut dragging_sprite: ResMut<DraggingSprite>,
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
    let characters = q_characters.single();
    for &character in characters.iter() {
        let (transform, mut character_component) = q_character.get_mut(character).unwrap();
        let half_width = Vec2::splat(CHARACTER_SCALE).x;
        let half_height = Vec2::splat(CHARACTER_SCALE).y;
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
}
