use egui::{ScrollArea, TextEdit, Ui};
use egui_extras::{Size, StripBuilder};
use tracing::info;

use crate::App;

#[derive(Clone, Debug, Default)]
pub(crate) struct Page1001 {
    user_account: String,
    user_nickname: String,
    user_name: String,
    user_phone: String,
    user_email: String,
}

pub(crate) fn show(app: &mut App, ui: &mut Ui) {
    ui.horizontal_wrapped(|ui| {
        let edit_width = 200.0;

        ui.label("账号：");
        TextEdit::singleline(&mut app.page1001.user_account)
            .desired_width(edit_width)
            .show(ui);

        ui.label("昵称：");
        TextEdit::singleline(&mut app.page1001.user_nickname)
            .desired_width(edit_width)
            .show(ui);

        ui.label("姓名：");
        TextEdit::singleline(&mut app.page1001.user_name)
            .desired_width(edit_width)
            .show(ui);

        ui.label("手机号：");
        if TextEdit::singleline(&mut app.page1001.user_phone)
            .desired_width(edit_width)
            .show(ui)
            .response
            .changed()
        {
            app.page1001.user_phone = app
                .page1001
                .user_phone
                .replace(|c| !char::is_numeric(c), "");
        };

        ui.label("邮箱：");
        TextEdit::singleline(&mut app.page1001.user_email)
            .desired_width(edit_width)
            .show(ui);

        if ui.button("查询").clicked() {
            info!("查询用户信息 {:?}", app.page1001);
        }
    });
    ui.separator();

    StripBuilder::new(ui)
        .size(Size::remainder())
        .vertical(|mut strip| {
            strip.cell(|ui| {
                ScrollArea::horizontal().show(ui, |ui| {
                    ui.label("121111111111111111111111111111111111111111111111111111111111111113");
                });
            })
        });
}
