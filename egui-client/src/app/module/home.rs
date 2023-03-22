use std::collections::HashMap;

use eframe::egui;
use eframe::egui::Context;
use egui::Ui;
use indextree::{Arena, NodeId};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use surf::http::convert::Serialize;
use surf::http::Method;
use surf::Request;
use tracing::{debug, error, info, warn};

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
pub(crate) struct Menu {
    pub menu_id: usize,
    pub parent_id: usize,
    pub menu_type: MenuType,
    pub menu_name: String,
    pub page_id: usize,
    #[serde(default = "expanded")]
    pub expanded: bool,
    #[serde(skip)]
    pub active: bool,
}

fn expanded() -> bool {
    true
}

pub(crate) struct Home {
    menus: Arena<Menu>,
    menu_map: HashMap<usize, NodeId>,
    page_id: usize,
}

impl Default for Home {
    fn default() -> Self {
        let mut menus = Arena::new();
        let mut menu_map = HashMap::new();
        let root = menus.new_node(Menu::default());
        menu_map.insert(0, root);

        Home {
            menus,
            menu_map,
            page_id: 0,
        }
    }
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
                match futures::executor::block_on(response.body_json::<Vec<Menu>>()) {
                    Ok(menu_res) => {
                        info!("获取菜单成功： {menu_res:?}");
                        let menus = &mut app.home.menus;
                        for menu in menu_res {
                            app.home.menu_map.insert(menu.menu_id, menus.new_node(menu));
                        }

                        let menu_map = &app.home.menu_map;
                        //遍历每一个菜单节点id
                        'LOOP: for (&child_menu_id, &child_node_id) in
                            menu_map.iter().filter(|(&id, _)| id != 0)
                        {
                            //找到这个节点
                            if let Some(child_node) = menus.get(child_node_id) {
                                let parent_menu_id = &child_node.get().parent_id;
                                //找到其父节点id
                                if let Some(parent_node_id) = menu_map.get(parent_menu_id) {
                                    debug!("{parent_menu_id} - {child_menu_id}");
                                    //遍历父节点id下的每一个子节点id
                                    for brother in parent_node_id.children(menus) {
                                        //找到菜单序号比当前菜单大的第一个,插到它前面，然后循环下一个菜单
                                        if menus.get(brother).unwrap().get().menu_id > child_menu_id
                                        {
                                            brother.insert_before(child_node_id, menus);
                                            continue 'LOOP;
                                        }
                                    }
                                    //父节点id下的每个子节点的菜单序号都比当前菜单小，所以把当前菜单插到最后
                                    parent_node_id.append(child_node_id, menus);
                                } else {
                                    warn!("没有找到{child_menu_id}的父菜单：{parent_menu_id}");
                                }
                            } else {
                                error!("没有找到当前菜单：{child_menu_id}");
                            }
                        }
                    }
                    Err(e) => error!("解析菜单数据失败: {e}"),
                }
            } else {
                error!("获取菜单失败： {response:?}");
            }
        }
        Err(e) => error!("获取菜单异常： {e}"),
    }
}

pub(crate) fn show(app: &mut App, ctx: &Context) {
    egui::SidePanel::left("menu").show(ctx, |ui| {
        let root = app.home.menu_map.get(&0).unwrap();
        let children = root.children(&app.home.menus).collect::<Vec<_>>();
        for child_node_id in children {
            show_menu(ui, app, child_node_id, 3.0);
        }
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        // ui.heading("欢迎来到我的主页");
        ui.label(app.home.page_id.to_string());
    });
}

fn show_menu(ui: &mut Ui, app: &mut App, menu_node_id: NodeId, indent: f32) {
    ui.vertical(|ui| {
        let menu = app.home.menus.get(menu_node_id).unwrap().get();
        ui.horizontal(|ui| {
            ui.add_space(indent);
            if menu.menu_type == MenuType::Item {
                if ui.button(&menu.menu_name).clicked() {
                    info!("{} Clicked", menu.menu_name);
                    app.home.page_id = menu.page_id;
                }
            } else {
                ui.label(&menu.menu_name);
            }
        });

        let children = menu_node_id.children(&app.home.menus).collect::<Vec<_>>();
        for child_node_id in children {
            show_menu(ui, app, child_node_id, 2.0 * indent);
        }
    });
}
