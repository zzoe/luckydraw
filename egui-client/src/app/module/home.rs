use eframe::egui;
use eframe::egui::Context;
use egui::Ui;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use surf::http::convert::Serialize;
use surf::http::Method;
use surf::Request;

use crate::app::PendingType;
use crate::App;

#[derive(PartialEq, Eq, Clone, Debug, Default, Deserialize_repr)]
#[repr(u8)]
pub enum MenuType {
    #[default]
    Label,
    Fold,
    Item,
}

#[derive(PartialEq, Eq, Clone, Debug, Default, Deserialize)]
pub(crate) struct MenuNode {
    pub menu_id: u32,
    pub parent_id: u32,
    pub menu_type: MenuType,
    pub menu_name: String,
    pub page_id: u32,
    #[serde(default = "expanded")]
    pub expanded: bool,
    #[serde(skip)]
    pub active: bool,
    pub children: Vec<MenuNode>,
}

impl MenuNode {
    fn show(self, ui: &mut Ui) {
        if self.menu_type == MenuType::Item {
            if ui.button(self.menu_name).clicked() {
                todo!()
            }
        } else {
            todo!()
        }
    }
}

fn expanded() -> bool {
    true
}

#[derive(Default)]
pub(crate) struct Home {
    menus: Vec<MenuNode>,
}

#[derive(Default, Serialize)]
struct MenuReq {
    sys: usize,
}

pub(crate) fn get_menu(app: &mut App) {
    let url = app.base_url.join("/api/menu").unwrap();
    let mut req = Request::new(Method::Get, url);
    req.set_query(&MenuReq { sys: 1 }).unwrap();

    let serial = app.next_serial();
    app.pending.insert(serial, PendingType::GetMenu);
    app.send(serial, req);
}

pub(crate) fn get_menu_callback(app: &mut App, res: surf::Result) {
    match res {
        Ok(mut response) => {
            if response.status().is_success() {
                match futures::executor::block_on(response.body_json::<Vec<MenuNode>>()) {
                    Ok(menus) => {
                        tracing::info!("获取菜单成功： {menus:?}");
                        app.home.menus = menus;
                    }
                    Err(e) => tracing::error!("解析菜单数据失败: {e}"),
                }
            } else {
                tracing::error!("获取菜单失败： {response:?}");
            }
        }
        Err(e) => tracing::error!("获取菜单异常： {e}"),
    }
}

pub(crate) fn show(app: &mut App, ctx: &Context) {
    egui::SidePanel::left("menu").show(ctx, |ui| {
        ui.vertical_centered_justified(|ui| {
            for menu in &app.home.menus {
                ui.label(&menu.menu_name);
            }
        });
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("欢迎来到我的主页");
    });
}
