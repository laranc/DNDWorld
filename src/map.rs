use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{
    check_file,
    components::{
        CharacterComponent, CharactersComponent, CustomMapComponent, MapComponent, MapFace,
        MapsComponent,
    },
    resources::{CurrentCustomMap, CustomMap, CustomMapList, ErrorLabel, LoadedFromFile},
    save::load_system,
    GameState,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CustomMap::default())
            .insert_resource(CustomMapList::default())
            .insert_resource(CurrentCustomMap::default())
            .add_startup_system_to_stage(
                StartupStage::PreStartup,
                map_setup_system.after(load_system),
            )
            .add_system(map_system)
            .add_system_set(
                SystemSet::on_update(GameState::CustomMap)
                    .with_system(load_custom_map_system)
                    .with_system(custom_map_list_system),
            );
    }
}

fn map_setup_system(mut commands: Commands, assets: Res<AssetServer>, loaded: Res<LoadedFromFile>) {
    if !loaded.0 {
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
}

fn map_system(
    state: Res<State<GameState>>,
    q_maps: Query<&Children, (With<MapsComponent>, Without<CharactersComponent>)>,
    mut q_map: Query<(&MapComponent, &mut Visibility), Without<CharacterComponent>>,
    mut q_custom_maps: Query<
        (&mut Visibility, &CustomMapComponent),
        (Without<CharacterComponent>, Without<MapComponent>),
    >,
    custom_map_list: Res<CustomMapList>,
    current_custom_map: Res<CurrentCustomMap>,
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
            _ => match map_component.face {
                MapFace::World => visibility.is_visible = false,
                MapFace::Town => visibility.is_visible = false,
            },
        }
    }
    if custom_map_list.1 > 0 {
        for (mut visibility, custom_map_component) in q_custom_maps.iter_mut() {
            for cm in custom_map_list.0.iter() {
                match &current_custom_map.0 {
                    Some(ccm) => {
                        if cm == ccm {
                            match cm.3 {
                                true => visibility.is_visible = false,
                                false => visibility.is_visible = true,
                            }
                        } else {
                            if cm == &custom_map_component.map {
                                visibility.is_visible = false;
                            }
                        }
                    }
                    None => {
                        visibility.is_visible = false;
                    }
                }
            }
        }
    }
}

fn load_custom_map_system(
    commands: Commands,
    assets: Res<AssetServer>,
    mut egui_ctx: ResMut<EguiContext>,
    mut custom_map: ResMut<CustomMap>,
    mut custom_map_list: ResMut<CustomMapList>,
    mut error_label: ResMut<ErrorLabel>,
) {
    egui::Window::new("Spawn Custom Maps").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("File name e.g. map.png (paste png file in assets/custom directory)");
        ui.text_edit_singleline(&mut custom_map.0);
        ui.label("Map name");
        ui.text_edit_singleline(&mut custom_map.1);
        ui.add(egui::Slider::new(&mut custom_map.2, 0.1..=50.).text("Initial Map Scale"));
        if ui.button("Spawn Map").clicked() {
            match check_file(&custom_map.0) {
                Some(s) => {
                    custom_map.0 = s;
                    spawn_custom_map(
                        commands,
                        assets,
                        &CustomMap(
                            custom_map.0.clone(),
                            custom_map.1.clone(),
                            custom_map.2,
                            custom_map.3,
                        ),
                    );
                    for cm in custom_map_list.0.iter_mut() {
                        cm.3 = true;
                    }
                    custom_map_list.0.push(CustomMap(
                        custom_map.0.clone(),
                        custom_map.1.clone(),
                        custom_map.2,
                        custom_map.3,
                    ));
                    custom_map_list.1 = custom_map_list.0.len();
                    error_label.update("".to_owned());
                }
                _ => {
                    error_label.update("Error finding file".to_owned());
                }
            }
        }
        error_label.content(ui);
    });
}

fn custom_map_list_system(
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    mut custom_map_list: ResMut<CustomMapList>,
    mut current_custom_map: ResMut<CurrentCustomMap>,
    q_custom_maps: Query<(Entity, &CustomMapComponent)>,
) {
    if custom_map_list.1 > 0 {
        egui::SidePanel::left("custom_map_list").show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.heading("Custom Maps");
                if ui.button("Delete all").clicked() {
                    for (e, _) in q_custom_maps.iter() {
                        commands.entity(e).despawn();
                    }
                    custom_map_list.0.clear();
                    custom_map_list.1 = 0;
                }
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut index = 0;
                let mut remove_list = Vec::new();
                for cm in custom_map_list.0.iter_mut() {
                    ui.horizontal(|ui| {
                        ui.label(&cm.1);
                        ui.toggle_value(&mut cm.3, "Hidden");
                        if !cm.3 {
                            current_custom_map.0 = Some(cm.clone());
                        }
                        match &current_custom_map.0 {
                            Some(ccm) => {
                                if ccm.1 == cm.1 && cm.3 {
                                    current_custom_map.0 = None;
                                }
                            }
                            _ => (),
                        }
                        if ui.button("Delete").clicked() {
                            for (e, cmc) in q_custom_maps.iter() {
                                if cmc.name == cm.1 {
                                    commands.entity(e).despawn();
                                    remove_list.push(index);
                                }
                            }
                        }
                    });
                    index += 1;
                }
                if remove_list.len() > 0 {
                    for i in remove_list {
                        custom_map_list.0.remove(i);
                    }
                }
                custom_map_list.1 = custom_map_list.0.len();
            });
        });
    }
}

pub fn spawn_custom_map(
    mut commands: Commands,
    assets: Res<AssetServer>,
    custom_map: &CustomMap,
) -> () {
    commands
        .spawn(SpriteBundle {
            texture: assets.load(format!("custom/{}", custom_map.0)),
            ..Default::default()
        })
        .insert(Name::new(custom_map.1.to_owned()))
        .insert(CustomMapComponent {
            name: custom_map.1.clone(),
            scale: Vec3::new(custom_map.2, custom_map.2, custom_map.2),
            map: custom_map.clone(),
        })
        .insert(SpatialBundle {
            visibility: Visibility { is_visible: true },
            transform: Transform {
                scale: Vec3::new(custom_map.2, custom_map.2, custom_map.2),
                ..Default::default()
            },
            ..Default::default()
        });
}
