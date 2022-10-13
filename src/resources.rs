use bevy::{asset::Handle, sprite::TextureAtlas};
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    WorldMap,
    TownMap,
    Interior,
}

#[derive(Inspectable, Default)]
pub struct SpriteSheet(pub Handle<TextureAtlas>);
