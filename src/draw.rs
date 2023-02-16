use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{
    resources::{CurrentCustomPainting, CustomPaintings},
    GameState,
};

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CustomPaintings::default())
            .insert_resource(CurrentCustomPainting::default())
            .add_system_set(
                SystemSet::on_update(GameState::Interior).with_system(draw_window_system),
            );
    }
}

fn draw_window_system(
    mut egui_ctx: ResMut<EguiContext>,
    mut custom_paintings: ResMut<CustomPaintings>,
    mut current_painting: ResMut<CurrentCustomPainting>,
) {
    egui::SidePanel::left("custom_map_list").show(egui_ctx.ctx_mut(), |ui| {
        for p in custom_paintings.0.iter() {
            if ui.button(&p.name).clicked() {
                current_painting.0 = p.clone();
            }
        }
    });
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.heading("Interiors:");
        ui.horizontal(|ui| {
            ui.label("Map name:");
            ui.text_edit_singleline(&mut current_painting.0.name);
        });
        ui.horizontal(|ui| {
            if ui.button("Save map").clicked() {
                let mut in_vec = false;
                for mut _p in custom_paintings.0.iter() {
                    if _p.name == current_painting.0.name {
                        _p = &current_painting.0.clone();
                        in_vec = true;
                    }
                }
                if !in_vec {
                    custom_paintings.0.push(current_painting.0.clone());
                }
            }
            if ui.button("Delete map").clicked() {
                custom_paintings
                    .0
                    .retain(|x| *x.name != current_painting.0.name);
            }
        });
        current_painting.0.control(ui);
        egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
            current_painting.0.content(ui);
        })
    });
}
