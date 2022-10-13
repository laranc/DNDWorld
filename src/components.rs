use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct CharacterComponent;

#[derive(Inspectable)]
pub enum MapFace {
    Front,
    Back,
}

#[derive(Component, Inspectable)]
pub struct MapComponent {
    pub face: MapFace,
}
