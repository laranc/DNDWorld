use bevy::prelude::{App, Plugin};
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};

use crate::{components::*, resources::*};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .add_plugin(InspectorPlugin::<SpriteSheet>::new())
                .register_inspectable::<CharacterComponent>()
                .register_inspectable::<MapComponent>()
                .register_inspectable::<CustomSpriteComponent>()
                .register_inspectable::<CustomMapComponent>()
                .add_plugin(InspectorPlugin::<CursorPosition>::new())
                .add_plugin(InspectorPlugin::<CustomMapList>::new())
                .add_plugin(InspectorPlugin::<CurrentCustomMap>::new());
        }
    }
}
