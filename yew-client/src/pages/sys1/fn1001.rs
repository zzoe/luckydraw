use yew::prelude::*;

pub struct Fn1001 {}

pub enum Msg {}

impl Component for Fn1001 {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Fn1001 {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="box">
                <div class="field is-grouped is-grouped-multiline">
                    <p class="control">
                        <input class="input" type="text" placeholder="Name" />
                    </p>
                    <p class="control">
                        <input class="input" type="phone" placeholder="Phone" />
                    </p>
                    <p class="control">
                        <input class="input" type="email" placeholder="Email" />
                    </p>
                    <p class="control">
                        <a class="button is-primary">{"Search"}</a>
                    </p>
                </div>
                <div class="table-container">
                    <table class="table is-bordered is-striped is-narrow is-hoverable is-fullwidth">
                        <thead>
                          <tr>
                            <th><abbr title="Position">{"Pos"}</abbr></th>
                            <th>{"Team"}</th>
                            <th><abbr title="Played">{"Pld"}</abbr></th>
                            <th><abbr title="Won">{"W"}</abbr></th>
                            <th><abbr title="Drawn">{"D"}</abbr></th>
                            <th><abbr title="Lost">{"L"}</abbr></th>
                            <th><abbr title="Goals for">{"GF"}</abbr></th>
                            <th><abbr title="Goals against">{"GA"}</abbr></th>
                            <th><abbr title="Goal difference">{"GD"}</abbr></th>
                            <th><abbr title="Points">{"Pts"}</abbr></th>
                            <th>{"Qualification or relegation"}</th>
                          </tr>
                        </thead>
                        <tbody>
                          <tr>
                            <td>{"1"}</td>
                            <td><a href="https://en.wikipedia.org/wiki/Leicester_City_F.C." title="Leicester City F.C.">{"Leicester City"}</a></td>
                            <td>{"38"}</td>
                            <td>{"23"}</td>
                            <td>{"12"}</td>
                            <td>{"3"}</td>
                            <td>{"68"}</td>
                            <td>{"36"}</td>
                            <td>{"+32"}</td>
                            <td>{"81"}</td>
                            <td>{"Qualification for the "}<a href="https://en.wikipedia.org/wiki/2016%E2%80%9317_UEFA_Champions_League#Group_stage" title="2016–17 UEFA Champions League">{"Champions League group stage"}</a></td>
                          </tr>
                        </tbody>
                    </table>
                </div>
                <nav class="level">
                    <div class="level-left" />
                    <div class="level-right">
                        <div class="level-item">
                            <div class="buttons has-addons">
                                <button class="button is-small">{"|<-"}</button>
                                <button class="button is-small">{"<<<"}</button>
                                <button class="button is-small">{">>>"}</button>
                                <button class="button is-small">{"->|"}</button>
                            </div>
                        </div>
                        <div class="level-item">
                            <div class="field has-addons">
                                <div class="control">
                                    <input class="input is-small" width="1rem" type="text" placeholder="Page No." />
                                </div>
                                <div class="control">
                                    <a class="button is-info is-small">{"Go"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="level-item">
                            <div class="field has-addons">
                                <div class="control">
                                    <input class="input is-small" type="text" placeholder="Rows/Page" />
                                </div>
                                <div class="control">
                                    <a class="button is-info is-small">{"Set"}</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </nav>
            </div>
        }
    }
}
