mod api;
mod cli;
mod config;
mod consts;
mod runtime;

use simplelog::{WriteLogger, LevelFilter};

use usdpl_back::Instance;
use usdpl_back::core::serdes::Primitive;

fn main() -> Result<(), ()> {
    let cli_args = cli::CliArgs::cli();
    let log_filepath = cli_args.log.unwrap_or_else(|| format!("/tmp/{}.log", consts::PACKAGE_NAME).into());
    WriteLogger::init(
        LevelFilter::Off,
        Default::default(),
        std::fs::File::create(&log_filepath).expect(&format!("Failed create log file {}", log_filepath.display()))
    ).unwrap();

    let filepath = cli_args.config.unwrap_or(consts::FILEPATH.into());

    let kaylon_conf = config::BaseConfig::load(&filepath);
    let (executor, sender) = runtime::RuntimeExecutor::new(kaylon_conf, filepath);

    log::info!("Starting back-end ({} v{})", consts::PACKAGE_NAME, consts::PACKAGE_VERSION);
    println!("Starting back-end ({} v{})", consts::PACKAGE_NAME, consts::PACKAGE_VERSION);
    let instance = Instance::new(consts::PORT)
        .register("hello", |_: Vec<Primitive>| vec![format!("Hello {}", consts::PACKAGE_NAME).into()])
        .register_blocking("get_about", api::get_about(sender.clone()))
        .register_async("get_display", api::GetDisplayEndpoint::new(sender.clone()))
        .register_async("get_javascript_to_run", api::GetJavascriptEndpoint::new(sender.clone()))
        .register_blocking("get_items", api::get_items(sender.clone()))
        .register("on_javascript_result", api::on_javascript_result(sender.clone()))
        .register("on_update", api::on_update(sender.clone()))
        .register_blocking("reload", api::reload(sender.clone()));
    let _exec_handle = executor.spawn();
    instance.run_blocking()
}
