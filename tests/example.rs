#![allow(warnings)]

use validex::{self as v, Check, DynError};

#[derive(Check)]
struct UserData {
    #[check(v::Any((
        v::Range(10..=20),
        30,
        v::Range(40..=50),
    )))]
    id: u32,
    #[check(url)]
    site: String,
    #[check(v::Range(13..=28), v::Not(v::Range(18..=24)))]
    age: u32,
}

fn url(_: &dyn AsRef<str>) -> Result<(), DynError<'_>> {
    // ...
    Ok(())
}

#[derive(Check)]
struct User {
    #[check(UserData::check)]
    data: UserData,
}

// #[test]
// fn example() -> v::Result {
//     let data = UserData {
//         id: 42,
//         site: Some("example.com".into()),
//         age: 25,
//     };
//     let binding = User { data };
//     let err = binding.check().err().unwrap();
//     println!("{:#}", err);
//     Ok(())
// }
