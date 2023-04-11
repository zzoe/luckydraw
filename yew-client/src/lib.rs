use std::cell::RefCell;
use std::rc::Rc;

use serde_repr::Deserialize_repr;
use yew::scheduler::Shared;
use yew::{classes, html, Component, Context, ContextProvider, Html};

use crate::context::{AppContext, AppScope, Module};
use crate::pages::login::Login;
use crate::pages::sys1::Sys1;

pub mod components;
mod context;
pub mod pages;

#[derive(Default)]
pub struct App {
    burger_switch: bool,
    context: Shared<AppContext>,
}

pub enum Msg {
    BurgerClick,
    Login(usize),
    SysClick(AppSys),
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum AppSys {
    #[default]
    Welcome,
    Sys1,
    Sys2,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut app_context = AppContext::default();
        app_context
            .scopes
            .insert(Module::Root, AppScope::new(Module::Root, ctx.link()));

        let context = Rc::new(RefCell::new(app_context));

        Self {
            context,
            ..Default::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::BurgerClick => {
                self.burger_switch = !self.burger_switch;
            }
            Msg::SysClick(sys_click) => {
                let sys = &mut self.context.borrow_mut().sys;
                if *sys == sys_click {
                    return false;
                }
                *sys = sys_click;
            }
            Msg::Login(userid) => {
                self.context.borrow_mut().userid = userid;
                log::info!("userid: {}", userid);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let context = Rc::clone(&self.context);
        let navbar_class = if self.burger_switch { "is-active" } else { "" };
        let burger_click = ctx.link().callback(|_| Msg::BurgerClick);
        let sys_click = |sys| ctx.link().callback(move |_| Msg::SysClick(sys));
        let sys_view = self.view_sys();
        let login = context.borrow().userid != 0;

        html! {
            <>
            <nav class="navbar px-6 py-5" role="navigation" aria-label="main navigation">
              <div class="navbar-brand">
                <a class="navbar-item" >
                  <img src="/resource/rustacean-flat-happy.svg" alt="" width="112" height="28" onclick={sys_click(AppSys::Welcome)} />
                </a>

                <a role="button" class={classes!("navbar-burger", navbar_class)} onclick={burger_click}
                    aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                </a>
              </div>

              if login {
              <div id="navbarBasicExample" class={classes!("navbar-menu", navbar_class)}>
                <div class="navbar-start">
                    <a class="navbar-item" onclick={sys_click(AppSys::Sys1)}>
                        { "Sys1" }
                    </a>
                    <a class="navbar-item" onclick={sys_click(AppSys::Sys2)}>
                        {"Sys2"}
                    </a>
                </div>

                <div class="navbar-end" />
              </div>
              }
            </nav>

            <main>
                <ContextProvider<Shared<AppContext>> context={context}>
                    { sys_view }
                </ContextProvider<Shared<AppContext>>>
            </main>

            <footer class="px-6 py-5">
                <div class="content has-text-centered">
                  <p>{"zoe © www.zoe.zz 鄂ICP备19880211号"}</p>
                </div>
            </footer>
            </>
        }
    }
}

impl App {
    fn view_sys(&self) -> Html {
        let context = RefCell::borrow(&self.context);
        if context.userid == 0 {
            return html! {
                <Login />
            };
        }

        match context.sys {
            AppSys::Welcome => html! {
                <h1> {"Welcome"} </h1>
            },
            AppSys::Sys1 => html! {
                <Sys1 />
            },
            AppSys::Sys2 => html! {"Sys 2"},
        }
    }
}
