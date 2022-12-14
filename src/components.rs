use bevy::prelude::{Component, Vec3};
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable, Default)]
pub struct CharacterComponent {
    pub is_hovered: bool,
}

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
pub struct CharactersComponent;

#[derive(Component)]
pub struct MapsComponent;

#[derive(Component, Default)]
pub struct CustomSpriteComponent {
    pub name: String,
    pub scale: Vec3,
    pub is_hovered: bool,
}
