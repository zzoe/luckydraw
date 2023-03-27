use egui::{TextEdit, Ui};
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};
use surf::http::Method;
use surf::Request;
use tracing::{error, info};

use crate::app::PendingType;
use crate::App;

#[derive(Clone, Debug, Default)]
pub(crate) struct Page1001 {
    user: User,
    users: Vec<User>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct User {
    user_account: String,
    user_nickname: String,
    user_name: String,
    user_phone: usize,
    user_email: String,
    #[serde(skip_serializing)]
    role_id: usize,
}

pub(crate) fn show(app: &mut App, ui: &mut Ui) {
    ui.horizontal_wrapped(|ui| {
        let edit_width = 200.0;

        ui.label("账号：");
        TextEdit::singleline(&mut app.page1001.user.user_account)
            .desired_width(edit_width)
            .show(ui);

        ui.label("昵称：");
        TextEdit::singleline(&mut app.page1001.user.user_nickname)
            .desired_width(edit_width)
            .show(ui);

        ui.label("姓名：");
        TextEdit::singleline(&mut app.page1001.user.user_name)
            .desired_width(edit_width)
            .show(ui);

        ui.label("手机号：");
        let mut user_phone = app.page1001.user.user_phone.to_string();
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
            app.page1001.user.user_phone = user_phone.parse().unwrap_or_default();
        };

        ui.label("邮箱：");
        TextEdit::singleline(&mut app.page1001.user.user_email)
            .desired_width(edit_width)
            .show(ui);

        if ui.button("查询").clicked() {
            info!("查询用户信息 {:?}", app.page1001);
            get_user(app);
        }
    });
    ui.separator();

    let height = ui.spacing().interact_size.y;
    TableBuilder::new(ui)
        .columns(Column::remainder(), 6)
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
        })
        .body(|mut body| {
            for user in &app.page1001.users {
                body.row(height, |mut row| {
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
                })
            }
        });
}

fn get_user(app: &mut App) {
    let url = app.base_url.join("/api/user").unwrap();
    let mut req = Request::new(Method::Post, url);
    req.body_json(&app.page1001.user).unwrap();

    let serial = app.next_serial();
    app.pending.insert(serial, PendingType::GetUser);
    app.send(serial, req);
}

pub(crate) fn get_user_callback(app: &mut App, res: surf::Result) {
    match res {
        Ok(mut response) => {
            if response.status().is_success() {
                match futures::executor::block_on(response.body_json::<Vec<User>>()) {
                    Ok(users) => app.page1001.users = users,
                    Err(e) => error!("解析用户数据失败: {e}"),
                }
            } else {
                error!("查询用户信息失败： {response:?}");
            }
        }
        Err(e) => error!("查询用户信息异常： {e}"),
    }
}
