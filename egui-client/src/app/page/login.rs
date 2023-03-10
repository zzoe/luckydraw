use eframe::egui;
use eframe::egui::Context;
use egui::{Button, FontSelection, Ui, Vec2, WidgetText};
use serde::Serialize;
use surf::http::Method;

use crate::app::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LoginStatus {
    Normal,
    Logging,
    Success,
}

impl Default for LoginStatus {
    fn default() -> Self {
        LoginStatus::Normal
    }
}

#[derive(Clone, Default, Debug)]
pub(crate) struct Login {
    user_account: String,
    password: String,
    font_size: Option<Vec2>,
    status: LoginStatus,
}

impl Login {
    pub(crate) fn text_size(&mut self, ui: &Ui, fallback_font: impl Into<FontSelection>) -> Vec2 {
        match self.font_size {
            Some(size) => size,
            None => {
                let font_size = WidgetText::from("用户:")
                    .into_galley(ui, None, 0.0, fallback_font)
                    .galley
                    .size();
                self.font_size = Some(font_size);
                font_size
            }
        }
    }
}

#[derive(Serialize)]
struct Args {
    user_account: String,
    password: String,
}

impl Args {
    fn new(user_account: String, password: String) -> Self {
        Args {
            user_account,
            password,
        }
    }
}

pub(crate) fn deal_response(app: &mut App, res: surf::Result) {
    match res {
        Ok(response) => {
            if response.status().is_success() {
                app.login.status = LoginStatus::Success;
                app.page = Page::Home;
                return;
            } else {
                log::error!("登录失败： {:?}", response);
            }
        }
        Err(e) => log::error!("登录异常： {e}"),
    }
    app.login.status = LoginStatus::Normal;
}

pub(crate) fn show(app: &mut App, ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.spacing_mut().item_spacing.y = 10.0;
            let space_height = (ui.available_height()
                - 6.0 * ui.spacing().item_spacing.y
                - 3.0 * ui.spacing().interact_size.y)
                / 2.0;

            ui.allocate_space(Vec2::new(ui.available_width(), space_height));

            let text_size = app.login.text_size(ui, TextStyle::Small);
            let space_width = (ui.available_width()
                - text_size.y
                - ui.spacing().item_spacing.x
                - ui.spacing().text_edit_width)
                / 2.0;

            ui.horizontal(|ui| {
                ui.add_space(space_width);
                ui.label("用户：");
                ui.text_edit_singleline(&mut app.login.user_account)
                    .changed();
            });

            ui.horizontal(|ui| {
                ui.add_space(space_width);
                ui.label("密码：");
                ui.text_edit_singleline(&mut app.login.password).changed();
            });

            if ui
                .add_enabled(app.login.status == LoginStatus::Normal, Button::new("登录"))
                .clicked()
            {
                app.login.status = LoginStatus::Logging;

                let url = app.base_url.join("/login").unwrap();
                let mut req = Request::new(Method::Post, url);
                let args = Args::new(app.login.user_account.clone(), app.login.password.clone());
                req.body_json(&args).unwrap();

                let serial = app.next_serial();
                app.pending.insert(serial, PendingType::Login);
                app.send(serial, req);
            }
        });

        ui.allocate_space(ui.available_size());
    });
}
