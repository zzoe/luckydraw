use std::fmt::{Display, Formatter};

use egui::{Align2, Context, ScrollArea, TextEdit, Ui, Vec2, Window};
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};
use surf::http::Method;
use surf::Request;
use tracing::{error, info};

use crate::app::module::page::Page;
use crate::app::{InnerHttp, PendingType};
use crate::App;

#[derive(Clone, Debug, Default)]
pub(crate) struct Page1001 {
    search_user: User,
    users: Vec<User>,
    modify_window: bool,
    delete_window: bool,
    user_row_index: usize,
}

pub(crate) fn show(app: &mut App, ctx: &Context, ui: &mut Ui) {
    let page1001 = &app.page.page1001;
    ui.add_enabled_ui(!page1001.modify_window && !page1001.delete_window, |ui| {
        show_search(app, ctx, ui);
        ui.separator();
        ScrollArea::both().show(ui, |ui| show_list(&mut app.page.page1001, ctx, ui));
    });
    show_modify_window(app, ctx, ui);
    show_delete_window(app, ctx, ui);
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct User {
    user_account: String,
    user_nickname: String,
    user_name: String,
    user_phone: u64,
    user_email: String,
    #[serde(skip_serializing)]
    role_id: usize,
}

fn show_search(app: &mut App, _ctx: &Context, ui: &mut Ui) {
    ui.horizontal_wrapped(|ui| {
        let App {
            page: Page { page1001, .. },
            ..
        } = app;

        let edit_width = 200.0;

        ui.label("账号：");
        TextEdit::singleline(&mut page1001.search_user.user_account)
            .desired_width(edit_width)
            .show(ui);

        ui.label("昵称：");
        TextEdit::singleline(&mut page1001.search_user.user_nickname)
            .desired_width(edit_width)
            .show(ui);

        ui.label("姓名：");
        TextEdit::singleline(&mut page1001.search_user.user_name)
            .desired_width(edit_width)
            .show(ui);

        ui.label("手机号：");
        let mut user_phone = page1001
            .search_user
            .user_phone
            .to_string()
            .trim_start_matches('0')
            .to_string();

        if TextEdit::singleline(&mut user_phone)
            .desired_width(edit_width)
            .show(ui)
            .response
            .changed()
        {
            user_phone = user_phone
                .replace(|c| !char::is_numeric(c), "")
                .trim_start_matches('0')
                .to_string();
            user_phone.truncate(11);
            page1001.search_user.user_phone = user_phone.parse().unwrap_or_default();
        };

        ui.label("邮箱：");
        TextEdit::singleline(&mut page1001.search_user.user_email)
            .desired_width(edit_width)
            .show(ui);

        if ui.button("查询").clicked() {
            info!("查询用户信息 {:?}", page1001.search_user);
            get_user(app);
        }
    });
}

fn show_list(page1001: &mut Page1001, _ctx: &Context, ui: &mut Ui) {
    let height = ui.spacing().interact_size.y;
    TableBuilder::new(ui)
        .columns(Column::remainder(), 7)
        .striped(true)
        .resizable(true)
        .header(height, |mut header| {
            header.col(|ui| {
                ui.heading("账号");
            });
            header.col(|ui| {
                ui.heading("昵称");
            });
            header.col(|ui| {
                ui.heading("姓名");
            });
            header.col(|ui| {
                ui.heading("手机");
            });
            header.col(|ui| {
                ui.heading("邮箱");
            });
            header.col(|ui| {
                ui.heading("角色ID");
            });
            header.col(|ui| {
                ui.heading("操作");
            });
        })
        .body(|body| {
            body.rows(height, page1001.users.len(), |row_index, mut row| {
                if let Some(user) = page1001.users.get(row_index) {
                    row.col(|ui| {
                        ui.label(&user.user_account);
                    });
                    row.col(|ui| {
                        ui.label(&user.user_nickname);
                    });
                    row.col(|ui| {
                        ui.label(&user.user_name);
                    });
                    row.col(|ui| {
                        ui.label(user.user_phone.to_string());
                    });
                    row.col(|ui| {
                        ui.label(&user.user_email);
                    });
                    row.col(|ui| {
                        ui.label(user.role_id.to_string());
                    });
                    row.col(|ui| {
                        ui.horizontal(|ui| {
                            if ui.button("修改").clicked() {
                                page1001.modify_window = true;
                                page1001.user_row_index = row_index;
                                info!("修改用户信息 {:?}", user);
                            }
                            if ui.button("删除").clicked() {
                                page1001.delete_window = true;
                                page1001.user_row_index = row_index;
                                info!("删除用户信息 {:?}", user);
                            }
                        });
                    });
                }
            });
        });
}

fn show_modify_window(app: &mut App, ctx: &Context, _ui: &mut Ui) {
    let App {
        inner_http,
        page: Page { page1001, .. },
    } = app;

    if page1001.modify_window {
        if let Some(user) = page1001.users.get(page1001.user_row_index) {
            let mut open = page1001.modify_window;
            Window::new("修改用户信息")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .open(&mut open)
                .show(ctx, |ui| {
                    let mut edit = String::from("edit");
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Label:");
                            ui.text_edit_singleline(&mut edit);
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Submit").clicked() {
                                modify_user(inner_http, user);
                                page1001.modify_window = false;
                            }
                            if ui.button("Close").clicked() {
                                page1001.modify_window = false;
                            }
                        });
                    });
                });
            page1001.modify_window &= open;
        }
    }
}

fn show_delete_window(app: &mut App, ctx: &Context, _ui: &mut Ui) {
    let App {
        inner_http,
        page: Page { page1001, .. },
    } = app;

    if page1001.delete_window {
        if let Some(user) = page1001.users.get(page1001.user_row_index) {
            let mut open = page1001.delete_window;
            Window::new("删除用户")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .open(&mut open)
                .show(ctx, |ui| {
                    if ui.button("确认").clicked() {
                        delete_user(inner_http, user);
                        page1001.delete_window = false;
                    }
                    if ui.button("取消").clicked() {
                        page1001.delete_window = false;
                    }
                });
            page1001.delete_window &= open;
        }
    }
}

fn get_user(app: &mut App) {
    let App {
        inner_http,
        page: Page { page1001, .. },
    } = app;
    let url = inner_http.base_url.join("/api/user").unwrap();
    let mut req = Request::new(Method::Get, url);

    req.set_query(&page1001.search_user).unwrap();
    inner_http.send(PendingType::GetUser, req);
}

pub(crate) fn get_user_callback(app: &mut App, res: surf::Result) {
    match res {
        Ok(mut response) => {
            if response.status().is_success() {
                match futures::executor::block_on(response.body_json::<Vec<User>>()) {
                    Ok(users) => {
                        app.page.page1001.users = users;
                        return;
                    }
                    Err(e) => error!("解析用户数据失败: {e}"),
                }
            }
            error!("查询用户信息失败： {response:?}");
        }
        Err(e) => error!("查询用户信息异常： {e}"),
    }
}

fn modify_user(inner_http: &mut InnerHttp, user: &User) {
    let url = inner_http.base_url.join("/api/user").unwrap();
    let mut req = Request::new(Method::Post, url);

    req.body_json(user).unwrap();
    inner_http.send(PendingType::ModifyUser, req);
}

fn delete_user(inner_http: &mut InnerHttp, user: &User) {
    let url = inner_http.base_url.join("/api/user").unwrap();
    let mut req = Request::new(Method::Delete, url);

    req.body_json(user).unwrap();
    inner_http.send(PendingType::DeleteUser, req);
}
