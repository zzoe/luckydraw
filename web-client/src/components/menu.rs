use std::collections::HashMap;

use gloo_net::http::Request;
use indextree::{Arena, NodeId};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use yew::{classes, html, html::Scope, Component, Context, Html, MouseEvent};

use crate::context::{ContextExt, Module};
use crate::pages::sys1::{Msg as Sys1Msg, Sys1};
use crate::AppSys;

#[derive(PartialEq, Eq, Clone, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum MenuType {
    Label,
    Fold,
    Item,
}

impl Default for MenuType {
    fn default() -> Self {
        MenuType::Label
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default, Deserialize)]
pub struct MenuNode {
    pub menu_id: u32,
    pub parent_id: u32,
    pub menu_type: MenuType,
    pub menu_name: String,
    pub func_id: u32,
    #[serde(default = "expanded")]
    pub expanded: bool,
    #[serde(skip)]
    pub active: bool,
}

fn expanded() -> bool {
    true
}

pub struct Menu {
    nodes: Arena<MenuNode>,
    node_map: HashMap<u32, NodeId>,
    activated: u32,
}

pub enum Msg {
    MenuInit(Vec<MenuNode>),
    MenuClicked(u32),
    MenuFail,
}

impl Component for Menu {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let context = ctx.context();
        let sys = context.borrow().sys as usize;
        let userid = context.borrow().userid;

        ctx.link().send_future(async move {
            if let Ok(res) = Request::get("/api/menu")
                .query([("sys", sys.to_string()), ("userid", userid.to_string())])
                .send()
                .await
            {
                match res.json::<Vec<MenuNode>>().await {
                    Ok(res) => return Msg::MenuInit(res),
                    Err(e) => {
                        log::error!("{e}");
                    }
                }
            }
            Msg::MenuFail
        });

        let mut nodes = Arena::new();
        let mut node_map = HashMap::new();
        let root = nodes.new_node(MenuNode::default());
        node_map.insert(0, root);

        Self {
            nodes,
            node_map,
            activated: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MenuInit(menu_res) => {
                let nodes = &mut self.nodes;
                let node_map = &mut self.node_map;

                for menu_node in &menu_res {
                    node_map.insert(menu_node.menu_id, nodes.new_node(menu_node.clone()));
                }

                for menu_node in &menu_res {
                    if let Some(parent) = node_map.get(&menu_node.parent_id) {
                        if let Some(child) = node_map.get(&menu_node.menu_id) {
                            parent.append(*child, nodes);
                        }
                    }
                }
            }
            Msg::MenuClicked(id) => {
                if id != self.activated {
                    if let Some(node_id) = self.node_map.get(&self.activated) {
                        if let Some(node) = self.nodes.get_mut(*node_id) {
                            node.get_mut().active = false;
                        }
                    }
                }

                let clicked = match self.node_map.get(&id) {
                    Some(node_id) => match self.nodes.get_mut(*node_id) {
                        Some(node) => node.get_mut(),
                        None => return false,
                    },
                    None => return false,
                };

                clicked.active = !clicked.active;
                clicked.expanded = !clicked.expanded;
                self.activated = id;

                if clicked.func_id > 0 {
                    let sys = ctx.context().borrow().sys;
                    match sys {
                        AppSys::Welcome => {}
                        AppSys::Sys1 => {
                            ctx.send::<Sys1>(Module::Sys1, Sys1Msg::MenuClicked(clicked.func_id))
                        }
                        AppSys::Sys2 => {}
                    }
                }
            }
            Msg::MenuFail => {
                log::info!("Get menu fail");
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let root = match self.node_map.get(&0) {
            Some(id) => id,
            None => return html! {},
        };

        html! {
            <aside class="menu">
                {for root.children(&self.nodes).map(|child| self.nodes.view(child, ctx.link()))}
            </aside>
        }
    }
}

trait MenuView {
    fn view(&self, node_id: NodeId, link: &Scope<Menu>) -> Html;
}

impl MenuView for Arena<MenuNode> {
    fn view(&self, node_id: NodeId, link: &Scope<Menu>) -> Html {
        let node = match self.get(node_id) {
            Some(node) => node.get(),
            None => return html! {},
        };

        let id = node.menu_id;
        let onclick = link.callback(move |_: MouseEvent| Msg::MenuClicked(id));
        let is_active = node.active.then_some("is-active");

        match node.menu_type {
            MenuType::Label => html! {
                <>
                <div class="menu-label" onclick={onclick}> {&*node.menu_name} </div>
                if node.expanded {
                     <ul class="menu-list">
                        { for node_id.children(self).map(|child| self.view(child, link)) }
                    </ul>
                }
                </>
            },
            MenuType::Fold => html! {
                <li>
                    <a onclick={onclick}>
                        {&*node.menu_name}
                    </a>
                    <ul>
                    if node.expanded {
                        { for node_id.children(self).map(|child| self.view(child, link)) }
                    }
                    </ul>
                </li>
            },
            MenuType::Item => html! {
                <li>
                    <a class={classes!(is_active)} onclick={onclick}>
                        {&*node.menu_name}
                    </a>
                </li>
            },
        }
    }
}
