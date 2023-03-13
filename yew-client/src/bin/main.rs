use web_client::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(tracing::Level::Debug));
    yew::Renderer::<App>::new().render();
}
