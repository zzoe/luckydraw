use eframe::egui::{Context, FontFamily, FontId, TextStyle};
use eframe::{egui, Frame};

use page::*;

mod page;

#[derive(Default)]
pub struct App {
    page: Page,
    login: login::Login,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        // egui_ctx.set_visuals(egui::Visuals::dark());
        // egui_ctx.set_debug_on_hover(true);

        App::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        match self.page {
            Page::Login => login::show(self, ctx),
            Page::Home => home::show(ctx),
        }
    }
}

fn setup_custom_fonts(ctx: &Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "consola".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/consola.ttf")),
    );
    fonts.font_data.insert(
        "simkai".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/simkai.ttf")),
    );

    let entry = fonts.families.entry(FontFamily::Proportional).or_default();
    entry.push("consola".to_owned());
    entry.push("simkai".to_owned());

    let entry = fonts.families.entry(FontFamily::Monospace).or_default();
    entry.push("consola".to_owned());
    entry.push("simkai".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);

    // Get current context style
    let mut style = (*ctx.style()).clone();

    // Redefine text_styles
    style.text_styles = [
        (TextStyle::Small, FontId::new(10.0, FontFamily::Monospace)),
        (TextStyle::Body, FontId::new(12.0, FontFamily::Monospace)),
        (
            TextStyle::Monospace,
            FontId::new(12.0, FontFamily::Monospace),
        ),
        (TextStyle::Button, FontId::new(12.0, FontFamily::Monospace)),
        (TextStyle::Heading, FontId::new(16.0, FontFamily::Monospace)),
    ]
    .into();

    // Mutate global style with above changes
    ctx.set_style(style);
}
