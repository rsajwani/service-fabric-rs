use fabric_rs::runtime::{
    executor::{DefaultExecutor, Executor},
    Runtime,
};
use log::info;
use windows_core::HSTRING;

mod echo;

fn main() -> windows::core::Result<()> {
    env_logger::init();

    info!("echomain start");

    let rt = tokio::runtime::Runtime::new().unwrap();
    let e = DefaultExecutor::new(rt.handle().clone());

    let runtime = Runtime::create(e.clone()).unwrap();
    let factory = echo::Factory::default();
    runtime
        .register_stateless_service_factory(&HSTRING::from("EchoAppService2"), factory)
        .unwrap();

    e.run_until_ctrl_c();
    Ok(())
}
