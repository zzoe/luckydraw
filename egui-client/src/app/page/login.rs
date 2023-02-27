use eframe::egui;
use eframe::egui::Context;
use egui::{Align, Grid, Layout};

use crate::app::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct Login {
    username: String,
    password: String,
}

pub(crate) fn show(app: &mut App, ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("用户：");
                    ui.text_edit_singleline(&mut app.login.username).changed();
                });

                ui.horizontal(|ui| {
                    ui.label("密码：");
                    ui.text_edit_singleline(&mut app.login.password).changed();
                });

                if ui.button("登录").clicked() {
                    app.page = Page::Home;
                }
            });
        });

        Grid::new("login_grid").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("用户：");
                ui.text_edit_singleline(&mut app.login.username).changed();
            });
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("密码：");
                ui.text_edit_singleline(&mut app.login.password).changed();
            });
            ui.end_row();

            if ui.button("登录").clicked() {
                app.page = Page::Home;
            }
            ui.end_row();
        });

        ui.allocate_space(ui.available_size());
    });
}
