use yew::prelude::*;

pub use fn1001::Fn1001;
pub use fn1002::Fn1002;

use crate::components::menu::Menu;
use crate::context::{ContextExt, Module};

pub mod fn1001;
pub mod fn1002;

#[derive(Debug, Default)]
pub struct Sys1 {
    page_id: u32,
}

pub enum Msg {
    MenuClicked(u32),
}

impl Component for Sys1 {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.insert_scope(Module::Sys1);
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MenuClicked(page_id) => self.page_id = page_id,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let func_view = self.view_func(ctx, self.page_id);

        html! {
            <div class="columns">
                <div class="column is-narrow">
                    <div class="box">
                        <Menu />
                    </div>
                </div>
                <div class="column">
                    {func_view}
                </div>
            </div>
        }
    }
}

impl Sys1 {
    fn view_func(&self, _ctx: &Context<Self>, func_id: u32) -> Html {
        match func_id {
            1001 => html! {
                <Fn1001 />
            },
            1002 => html! {
                <Fn1002 />
            },
            _ => html! {
                <h1>{"Welcome to Sys 1"}</h1>
            },
        }
    }
}
