use bevy::prelude::*;

use crate::components::{MapComponent, MapFace};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) -> () {
        app.add_startup_system_to_stage(StartupStage::PreStartup, map_setup_system);
    }
}

fn map_setup_system(mut commands: Commands, assets: Res<AssetServer>) {
    let map_front_sprite = SpriteBundle {
        texture: assets.load("map/front.png"),
        ..Default::default()
    };
    let map_back_sprite = SpriteBundle {
        texture: assets.load("map/back.png"),
        ..Default::default()
    };
    let map_front = commands
        .spawn_bundle(map_front_sprite)
        .insert(Name::new("World Map"))
        .insert(MapComponent {
            face: MapFace::Front,
        })
        .id();
    let map_back = commands
        .spawn_bundle(map_back_sprite)
        .insert(Name::new("Town Map"))
        .insert(MapComponent {
            face: MapFace::Back,
        })
        .id();
    commands
        .spawn()
        .insert(Name::new("Maps"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Visibility::visible())
        .push_children(&vec![map_front, map_back]);
}
