use bevy::{asset::Handle, prelude::Vec2, sprite::TextureAtlas};
use bevy_egui::egui;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    WorldMap,
    TownMap,
    Interior,
}

#[derive(Inspectable, Default)]
pub struct SpriteSheet(pub Handle<TextureAtlas>);

#[derive(Clone)]
pub struct Painting {
    pub lines: Vec<Vec<egui::Vec2>>,
    pub stroke: egui::Stroke,
    pub name: String,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            stroke: egui::Stroke::new(1., egui::Color32::WHITE),
            name: String::default(),
        }
    }
}

impl Painting {
    pub fn control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            egui::stroke_ui(ui, &mut self.stroke, "Stroke");
            ui.separator();
            if ui.button("Clear painting").clicked() {
                self.lines.clear();
            }
        })
        .response
    }

    pub fn content(&mut self, ui: &mut egui::Ui) {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::drag());
        let rect = response.rect;
        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }
        let current_line = self.lines.last_mut().unwrap();
        if let Some(p_pos) = response.interact_pointer_pos() {
            let canvas_pos = p_pos - rect.min;
            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
            }
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
        }
        for line in &self.lines {
            if line.len() >= 2 {
                let points: Vec<egui::Pos2> = line.iter().map(|p| rect.min + *p).collect();
                painter.add(egui::Shape::line(points, self.stroke));
            }
        }
    }
}

#[derive(Default)]
pub struct CustomMaps(pub Vec<Painting>);

#[derive(Default)]
pub struct CurrentCustomMap(pub Painting);

#[derive(Default, Inspectable)]
pub struct CursorPosition(pub Vec2);

#[derive(Default)]
pub struct DraggingSprite(pub bool);

pub struct CustomSprite(pub String, pub String, pub f32);

impl Default for CustomSprite {
    fn default() -> Self {
        Self(String::default(), String::default(), 1.)
    }
}

#[derive(Default)]
pub struct CustomSpriteList(pub Vec<String>);

#[derive(Default)]
pub struct CustomSpriteNum(pub u8);
