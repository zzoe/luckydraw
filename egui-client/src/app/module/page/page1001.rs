use crate::App;
use egui::Ui;
use tracing::info;

#[derive(Clone, Debug, Default)]
pub(crate) struct Page1001 {
    username: String,
}

pub(crate) fn show(app: &mut App, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.text_edit_singleline(&mut app.page1001.username);
        if ui.button("查询").clicked() {
            info!("查询用户信息 {}", app.page1001.username);
        }
        ui.separator();
    });
}
