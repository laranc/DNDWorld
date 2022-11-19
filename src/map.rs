use bevy::prelude::*;

use crate::{
    components::{CharacterComponent, CharactersComponent, MapComponent, MapFace, MapsComponent},
    GameState,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, map_setup_system)
            .add_system(map_system);
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
        .spawn(map_front_sprite)
        .insert(Name::new("World Map"))
        .insert(MapComponent {
            face: MapFace::World,
        })
        .id();
    let map_back = commands
        .spawn(map_back_sprite)
        .insert(Name::new("Town Map"))
        .insert(MapComponent {
            face: MapFace::Town,
        })
        .insert(Transform {
            scale: Vec3::new(0.4, 0.4, 0.4),
            ..Default::default()
        })
        .id();
    commands
        .spawn_empty()
        .insert(Name::new("Maps"))
        .insert(MapsComponent)
        .insert(SpatialBundle {
            visibility: Visibility { is_visible: true },
            ..Default::default()
        })
        .push_children(&vec![map_front, map_back]);
}

fn map_system(
    state: Res<State<GameState>>,
    q_maps: Query<&Children, (With<MapsComponent>, Without<CharactersComponent>)>,
    mut q_map: Query<(&MapComponent, &mut Visibility), Without<CharacterComponent>>,
) {
    let maps = q_maps.single();
    for &map in maps.iter() {
        let (map_component, mut visibility) = q_map.get_mut(map).unwrap();
        match state.current() {
            GameState::WorldMap => match map_component.face {
                MapFace::World => visibility.is_visible = true,
                MapFace::Town => visibility.is_visible = false,
            },
            GameState::TownMap => match map_component.face {
                MapFace::World => visibility.is_visible = false,
                MapFace::Town => visibility.is_visible = true,
            },
            GameState::Interior => match map_component.face {
                MapFace::World => visibility.is_visible = false,
                MapFace::Town => visibility.is_visible = false,
            },
        }
    }
}
