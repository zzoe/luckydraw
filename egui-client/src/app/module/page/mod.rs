use egui::Ui;

use crate::App;

pub(crate) mod page1001;

pub(crate) fn show(app: &mut App, ui: &mut Ui) {
    match app
        .home
        .active_node_id
        .and_then(|active| app.home.menus.get(active))
        .map(|menu| menu.get().page_id)
        .unwrap_or_default()
    {
        1001 => page1001::show(app, ui),
        1002 => {
            ui.heading("1002");
        }
        1003 => {
            ui.heading("1003");
        }
        _ => {
            ui.heading("欢迎来到我的主页");
        }
    }
}
