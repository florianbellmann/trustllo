use std::rc::Rc;

use application::app::ApplicationService;
use infrastructure::repositories::api_repository::ApiKanbanRepository;
use utils::logger::Logger;

mod application;
mod domain;
mod infrastructure;
mod utils;

#[tokio::main]
async fn main() {
    Logger::init();

    let kanban_repo = Rc::new(ApiKanbanRepository::new());

    let mut application_service = ApplicationService::new(kanban_repo);

    application_service.init().await;

    application_service.run_app_loop().await;

    application_service.teardown();
}

//pub trait Foo {
//    // set of methods implemented by the trait
//    fn foo(&self) -> String;
//}

//struct Bar {
//    // data structure
//    x: String,
//}

//impl Foo for Bar {
//    //implement the trait for the data structure
//    fn foo(&self) -> String {
//        self.x.clone()
//    }
//}

//struct Baz {
//    x: String,
//}

//impl Baz {
//    fn new() -> Baz {
//        Baz {
//            x: "hello".to_string(),
//        }
//    }

//    fn fsdjk(&self, fjdk: &dyn Foo) -> String {
//        "dajk".to_string()
//    }
//}

//fn main() {
//    let bar = Bar {
//        x: "hello".to_string(),
//    };

//    let baz = Baz::new();

//    baz.fsdjk(&bar);
//}

//pub trait Foo2: {
//    fn foo(&self) -> String;
//}
//impl dyn Foo2 {
//    fn foo(&self) -> String {
//        "hello".to_string()
//    }
//}

//struct ASJKL {
//    x: String,
//}
