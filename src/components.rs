use bevy::prelude::{Component, Vec3};
use bevy_inspector_egui::Inspectable;

use crate::resources::CustomMap;

#[derive(Component, Inspectable, Default)]
pub struct CharacterComponent {
    pub is_hovered: bool,
}

#[derive(Component)]
pub struct CharactersComponent;

#[derive(Inspectable, PartialEq)]
pub enum MapFace {
    World,
    Town,
}

#[derive(Component, Inspectable)]
pub struct MapComponent {
    pub face: MapFace,
}

#[derive(Component)]
pub struct MapsComponent;

#[derive(Component, Inspectable, Default)]
pub struct CustomSpriteComponent {
    pub name: String,
    pub scale: Vec3,
    pub is_hovered: bool,
}

#[derive(Component, Inspectable, Default)]
pub struct CustomMapComponent {
    pub name: String,
    pub scale: Vec3,
    pub map: CustomMap,
}
