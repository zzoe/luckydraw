use eframe::egui;
use eframe::egui::Context;
use egui::{Button, FontSelection, Key, TextEdit, Ui, Vec2, Widget, WidgetText};
use serde::Serialize;
use surf::http::Method;
use tracing::error;

use crate::app::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) enum LoginStatus {
    #[default]
    Normal,
    Logging,
    Success,
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

fn login(app: &App) {
    let url = app.base_url.join("/login").unwrap();
    let mut req = Request::new(Method::Post, url);
    let args = Args::new(app.login.user_account.clone(), app.login.password.clone());

    req.body_json(&args).unwrap();
    app.send(PendingType::Login, req);
}

pub(crate) fn login_callback(app: &mut App, res: surf::Result) {
    match res {
        Ok(response) => {
            if response.status().is_success() {
                app.login.status = LoginStatus::Success;
                app.module = Module::Home;
                home::get_menu(app);
                return;
            }
            error!("登录失败： {:?}", response);
        }
        Err(e) => error!("登录异常： {e}"),
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
                TextEdit::singleline(&mut app.login.password)
                    .password(true)
                    .ui(ui);
            });

            let btn = ui.add_enabled(app.login.status == LoginStatus::Normal, Button::new("登录"));
            if btn.clicked() || ctx.input(|i| i.key_pressed(Key::Enter)) {
                app.login.status = LoginStatus::Logging;
                login(app);
            }
        });

        ui.allocate_space(ui.available_size());
    });
}
