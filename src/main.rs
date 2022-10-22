use app::ApplicationService;

mod app;
mod trello;


#[tokio::main]
async fn main() {

    let application_service = ApplicationService::new();

    // application_service::init

    application_service::run_app_loop();

    application_service::teardown();
}
