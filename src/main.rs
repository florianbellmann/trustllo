
use app::ApplicationService;
use log::{info, error};

use crate::utils::logger::Logger;

mod app;
mod config;
mod store;
mod trello;
mod ui;
mod utils;

#[tokio::main]
async fn main() {
    Logger::init();

    info!("booting up");
    error!("booting up");
    let application_service = ApplicationService::new();

    application_service.init().await;

    application_service.run_app_loop();

    application_service.teardown();
}
