use caylon_studio::ui::App;

#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_log::init(wasm_log::Config::default());
    log::info!("Hydrating UI");
    yew::Renderer::<App>::new().hydrate();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    compile_error!("frontend is for browsers (wasm32)");
}
