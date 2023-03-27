use std::collections::HashMap;
use std::time::Duration;

use async_channel::{unbounded, Receiver, Sender};
use eframe::egui::{Context, FontFamily, FontId, TextStyle};
use eframe::{egui, Frame};
use surf::{Client, Config, Request, Url};

use module::*;

mod module;

pub struct App {
    serial: usize,
    pending: HashMap<usize, PendingType>,
    unbounded_channel: UnboundedChannel,
    base_url: Url,
    client: Client,
    module: Module,
    login: login::Login,
    home: home::Home,
    page1001: page::page1001::Page1001,
}

impl Default for App {
    fn default() -> Self {
        App {
            serial: Default::default(),
            pending: Default::default(),
            unbounded_channel: Default::default(),
            base_url: Url::parse("https://127.0.0.1:1314/").unwrap(),
            client: Default::default(),
            module: Default::default(),
            login: Default::default(),
            home: Default::default(),
            page1001: Default::default(),
        }
    }
}

pub(crate) enum PendingType {
    Login,
    GetMenu,
    GetUser,
}

#[derive(Clone)]
pub(crate) struct UnboundedChannel {
    sender: Sender<AsyncResult>,
    receiver: Receiver<AsyncResult>,
}

impl Default for UnboundedChannel {
    fn default() -> Self {
        let (sender, receiver) = unbounded();
        UnboundedChannel { sender, receiver }
    }
}

pub(crate) struct AsyncResult {
    serial: usize,
    res: surf::Result,
}

impl AsyncResult {
    pub fn new(serial: usize, res: surf::Result) -> Self {
        AsyncResult { serial, res }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let egui_ctx = &cc.egui_ctx;
        setup_custom_fonts(egui_ctx);
        // egui_ctx.set_visuals(egui::Visuals::dark());
        // egui_ctx.set_debug_on_hover(true);

        let client: Client = Config::new()
            .set_timeout(Some(Duration::from_secs(30)))
            .try_into()
            .unwrap();

        App {
            client,
            ..Default::default()
        }
    }

    pub fn next_serial(&mut self) -> usize {
        self.serial += 1;
        self.serial
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn send(&self, serial: usize, req: Request) {
        let client = self.client.clone();
        let sender = self.unbounded_channel.sender.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let res = client.send(req).await;
            if let Err(e) = sender.send(AsyncResult::new(serial, res)).await {
                tracing::error!("收到异步响应后，往通道发送结果时报错 {serial}： {e}");
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn send(&self, serial: usize, req: Request) {
        let client = self.client.clone();
        let sender = self.unbounded_channel.sender.clone();
        async_global_executor::spawn(async move {
            tracing::info!("{serial}: {req:?}");
            let res = client.send(req).await;
            if let Err(e) = sender.send(AsyncResult::new(serial, res)).await {
                tracing::error!("收到异步响应后，往通道发送结果时报错 {serial}： {e}");
            };
        })
        .detach();
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        while let Ok(msg) = self.unbounded_channel.receiver.try_recv() {
            if let Some(pt) = self.pending.remove(&msg.serial) {
                match pt {
                    PendingType::Login => login::login_callback(self, msg.res),
                    PendingType::GetMenu => home::get_menu_callback(self, msg.res),
                    PendingType::GetUser => page::page1001::get_user_callback(self, msg.res),
                }
            }
        }

        match self.module {
            Module::Login => login::show(self, ctx),
            Module::Home => home::show(self, ctx),
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

    // let entry = fonts.families.entry(FontFamily::Proportional).or_default();
    // entry.push("consola".to_owned());
    // entry.push("simkai".to_owned());

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
