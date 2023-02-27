use eframe::egui;
use eframe::egui::Context;

pub(crate) fn show(ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("欢迎来到我的主页");
    });
}
