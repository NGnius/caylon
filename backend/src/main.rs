mod api;
mod config;
mod consts;
mod runtime;

use simplelog::{WriteLogger, LevelFilter};

use usdpl_back::Instance;
use usdpl_back::core::serdes::Primitive;

fn main() -> Result<(), ()> {
    let log_filepath = format!("/tmp/{}.log", consts::PACKAGE_NAME);
    WriteLogger::init(
        #[cfg(debug_assertions)]{LevelFilter::Debug},
        #[cfg(not(debug_assertions))]{LevelFilter::Info},
        Default::default(),
        std::fs::File::create(&log_filepath).unwrap()
    ).unwrap();

    let kaylon_conf = config::BaseConfig::load(consts::FILEPATH);
    let (executor, sender) = runtime::RuntimeExecutor::new(kaylon_conf);

    log::info!("Starting back-end ({} v{})", consts::PACKAGE_NAME, consts::PACKAGE_VERSION);
    println!("Starting back-end ({} v{})", consts::PACKAGE_NAME, consts::PACKAGE_VERSION);
    let instance = Instance::new(consts::PORT)
        .register("hello", |_: Vec<Primitive>| vec![format!("Hello {}", consts::PACKAGE_NAME).into()])
        .register_blocking("get_about", api::get_about(sender.clone()))
        .register_async("get_display", api::GetDisplayEndpoint::new(sender.clone()))
        .register_blocking("get_items", api::get_items(sender.clone()))
        .register("on_update", api::on_update(sender.clone()))
        .register_blocking("reload", api::reload(sender.clone()));
    let _exec_handle = executor.spawn();
    instance.run_blocking()
}
