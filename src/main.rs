mod link_list;
mod tree;

use std::cell::RefCell;

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
    // Many other fields
}

fn main() {
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };
    let date = 2020;

    user_1.active
          .replace_with(|_| if date < 2000 { true } else { false });
    println!("{:?}", user_1.active);
}

