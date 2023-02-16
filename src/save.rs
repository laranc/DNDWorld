use bevy::{app::AppExit, ecs::system::SystemState, prelude::*};
use std::{fs::File, io::Write, path::Path};

use crate::resources::LoadedFromFile;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveExitEvent>()
            .insert_resource(LoadedFromFile::default())
            .add_startup_system_to_stage(StartupStage::PreStartup, load_system)
            .add_system(save_system);
    }
}

const SCENE_PATH: &str = "saves/scene.scn.ron";

pub fn load_system(
    assets: Res<AssetServer>,
    mut spawner: ResMut<SceneSpawner>,
    mut loaded: ResMut<LoadedFromFile>,
) {
    if Path::new(SCENE_PATH).exists() {
        spawner.spawn(assets.load(SCENE_PATH));
        loaded.0 = true;
    };
}

/*
pub fn load_system(world: &mut World) {
    if Path::new(SCENE_PATH).exists() {
        let assets = world.resource::<AssetServer>();
        let mut spawner = world.resource_mut::<SceneSpawner>();
        spawner.spawn(assets.load(SCENE_PATH));
        let mut loaded = world.resource_mut::<LoadedFromFile>();
        loaded.0 = true;
    }
}
*/
pub struct SaveExitEvent;

fn save_system(world: &mut World) {
    let mut ev_save_exit: SystemState<EventReader<SaveExitEvent>> = SystemState::new(world);
    if !ev_save_exit.get_mut(world).is_empty() {
        let type_registry = world.resource::<AppTypeRegistry>();
        let scene = DynamicScene::from_world(&world, type_registry);
        let serialized_scene = scene.serialize_ron(type_registry).unwrap();

        File::create(format!("assets/{}", SCENE_PATH))
            .and_then(|mut file| file.write(serialized_scene.as_bytes()))
            .expect("Error saving scene");

        world.resource_mut::<Events<AppExit>>().send(AppExit);
    }
}
