use app::ApplicationService;

mod app;
mod trello;
mod config;
mod utils;
mod ui;

#[tokio::main]
async fn main() {
    let application_service = ApplicationService::new();

    application_service.init().await;

    application_service.run_app_loop();

    application_service.teardown();
}
