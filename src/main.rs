use app::ApplicationService;

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

    let application_service = ApplicationService::new();

    application_service.init().await;

    application_service.run_app_loop();

    application_service.teardown();
}
