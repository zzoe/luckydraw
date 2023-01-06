use yew::prelude::*;

pub struct Fn1002;
pub enum Msg {}

impl Component for Fn1002 {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="box">
                <div class="tabs is-toggle">
                    <ul>
                        <li class="is-active"><a>{"Pictures"}</a></li>
                        <li><a>{"Music"}</a></li>
                        <li><a>{"Videos"}</a></li>
                        <li><a>{"Documents"}</a></li>
                    </ul>
                </div>
            </div>
        }
    }
}
