// use fake::{Dummy, Fake, Faker, ResultFaker, StringFaker};
use fake::faker::boolean::en::Boolean;
use fake::faker::lorem::en::Words;

use fake::{Fake, Faker};

use crate::domain::entities::{Board, Card, List};

// TODO: remove this file from the normal app module tree. only for testing

pub struct FakeData {}

impl FakeData {
    pub fn get_fake_board() -> Board {
        Board {
            name: get_random_string(3..5),
            desc: get_random_string(10..15),
            closed: Boolean(50).fake(),
            id: Faker.fake::<String>(),
            url: format!("https://{}/test/path", Faker.fake::<String>()),
            subscribed: Boolean(50).fake(),
        }
    }

    pub fn get_fake_list() -> List {
        List {
            id: Faker.fake::<String>(),
            name: get_random_string(3..5),
            closed: Boolean(50).fake(),
            idBoard: Faker.fake::<String>(),
            subscribed: Boolean(50).fake(),
        }
    }

    pub fn get_fake_card() -> Card {
        Card {
            id: Faker.fake::<String>(),
            name: get_random_string(3..5),
            closed: Boolean(50).fake(),
            desc: get_random_string(10..15),
            idBoard: Faker.fake::<String>(),
            idList: Faker.fake::<String>(),
            manualCoverAttachment: Boolean(50).fake(),
            shortLink: Faker.fake::<String>(),
            dueComplete: Boolean(50).fake(),
            shortUrl: Faker.fake::<String>(),
            subscribed: Boolean(50).fake(),
            url: Faker.fake::<String>(),
        }
    }

    // pub fn get_fake_store_data() -> StoreData {
    //     let boards = vec![
    //         Self::get_fake_board(),
    //         Self::get_fake_board(),
    //         Self::get_fake_board(),
    //     ];

    //     let lists = vec![
    //         Self::get_fake_list(),
    //         Self::get_fake_list(),
    //         Self::get_fake_list(),
    //         Self::get_fake_list(),
    //     ];

    //     StoreData {
    //         updated: "missing date".to_string(), //TODO: not implemented yet
    //         boards,
    //         lists,
    //     }
    // }
}

fn get_random_string(count: std::ops::Range<usize>) -> String {
    let words: Vec<String> = Words(count).fake();
    words.concat()
}
