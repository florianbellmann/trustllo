// use fake::{Dummy, Fake, Faker, ResultFaker, StringFaker};
use crate::trello::Board;
use fake::faker::lorem::en::Words;
use fake::uuid::{UUIDv1, UUIDv4};
use fake::{Fake, Faker};

pub fn get_fake_board() -> Board {
    Board {
        name: get_random_string(3..5),
        desc: get_random_string(10..15),
        closed: false,
        id: Faker.fake::<String>(),
        url: format!("https://{}/test/path", Faker.fake::<String>()),
        subscribed: false,
    }
}

fn get_random_string(count: std::ops::Range<usize>) -> String {
    let mut words: Vec<String> = Words(count).fake();
    words.concat()
}
