use egui::{Context, Ui};

use crate::app::module::home::Home;
use crate::app::module::login::Login;
use crate::app::module::Module;
use crate::App;

pub(crate) mod page1001;

#[derive(Default)]
pub(crate) struct Page {
    pub(crate) module: Module,
    pub(crate) login: Login,
    pub(crate) home: Home,
    pub(crate) page1001: page1001::Page1001,
}

pub(crate) fn show(app: &mut App, ctx: &Context, ui: &mut Ui) {
    let App {
        page: Page { home, .. },
        ..
    } = app;

    match home
        .active_node_id
        .and_then(|active| home.menus.get(active))
        .map(|menu| menu.get().page_id)
        .unwrap_or_default()
    {
        1001 => page1001::show(app, ctx, ui),
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
