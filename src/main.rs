mod app;
mod trello;

use app::ApplicationService;

fn main() {
    ApplicationService::init();

    ApplicationService::run_app_loop();

    ApplicationService::teardown();
}
