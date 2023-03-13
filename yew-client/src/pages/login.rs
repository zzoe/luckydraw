use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, NodeRef};

use crate::context::{ContextExt, Module};
use crate::{App, Msg as RootMsg};

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Login {
    user_account: String,
    password: String,
    #[serde(skip)]
    loading: bool,
    #[serde(skip)]
    user_ref: NodeRef,
    #[serde(skip)]
    pass_ref: NodeRef,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct LoginRes {
    userid: usize,
}

pub enum Msg {
    BtnClicked,
    LoginResponse(LoginRes),
    LoginFail,
    UserInput(String),
    PassInput(String),
    InputErr,
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Login::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::BtnClicked => {
                self.loading = true;

                let login_req = self.clone();
                ctx.link().send_future(async move {
                    if let Ok(req) = Request::post("/login").json(&login_req) {
                        if let Ok(res) = req.send().await {
                            if let Ok(res) = res.json::<LoginRes>().await {
                                return Msg::LoginResponse(res);
                            }
                        }
                    }
                    Msg::LoginFail
                });
            }
            Msg::LoginResponse(res) => {
                ctx.send::<App>(Module::Root, RootMsg::Login(res.userid));
            }
            Msg::LoginFail => {
                self.loading = false;
            }
            Msg::UserInput(user_account) => self.user_account = user_account,
            Msg::PassInput(password) => self.password = password,
            Msg::InputErr => {
                tracing::info!("Input Error");
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::BtnClicked);
        let disabled = self.loading;

        let user_ref = self.user_ref.clone();
        let on_user_input = ctx.link().callback(move |_| {
            let input = user_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                return Msg::UserInput(input.value());
            }

            Msg::InputErr
        });

        let pass_ref = self.pass_ref.clone();
        let on_pass_input = ctx.link().callback(move |_| {
            let input = pass_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                return Msg::PassInput(input.value());
            }

            Msg::InputErr
        });

        html! {
        <div class="box">
          <div class="field">
            <label class="label">{"账号"}</label>
            <div class="control">
              <input class="input" type="text" ref={self.user_ref.clone()} onchange={on_user_input} />
            </div>
          </div>

          <div class="field">
            <label class="label">{"密码"}</label>
            <div class="control">
              <input class="input" type="password" ref={self.pass_ref.clone()} onchange={on_pass_input} />
            </div>
          </div>

          <button class="button is-primary" type="button" {disabled} {onclick}>
            if self.loading{
              {"加载中..."}
            } else{
              {"登录"}
            }
          </button>
        </div>
        }
    }
}
