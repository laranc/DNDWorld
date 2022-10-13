use bevy::prelude::*;

use crate::components::{CharacterComponent, MapComponent, MapFace};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) -> () {
        app.add_system(camera_system);
    }
}

fn camera_system(
    mut camera_query: Query<&mut Transform, (Without<MapComponent>, Without<CharacterComponent>)>,
    maps_query: Query<&Children, (Without<MapComponent>, Without<CharacterComponent>)>,
    map_query: Query<(&Transform, &MapComponent), Without<CharacterComponent>>,
) {
    let mut camera_transform = camera_query.single_mut();
    let maps = maps_query.single();
    for &map in maps.iter() {
        let (map_transform, map_component) = map_query.get(map).unwrap();
        match map_component.face {
            MapFace::Front => {}
            MapFace::Back => {}
        }
    }
}
