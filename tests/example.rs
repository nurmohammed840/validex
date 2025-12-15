use validex::{self as v, Check};

#[derive(Check)]
struct UserData {
    #[check(v::Any((
        v::Range(10..=20),
        30,
        v::Range(40..=50),
    )))]
    id: u32,
    #[check(v::Maybe((
        v::Not("example.com"),
        v::Length(..=20),
        url,
    )))]
    site: Option<String>,
    #[check(v::Range(13..=28), v::Not(v::Range(18..=24)))]
    age: u32,
}

fn url<T: AsRef<str>>(_: &T) -> v::Result {
    // ...
    Ok(())
}

#[derive(Check)]
struct User {
    #[check(UserData::check)]
    data: UserData,
}

#[test]
fn example() -> v::Result {
    let data = UserData {
        id: 42,
        site: Some("example.com".into()),
        age: 25,
    };
    let aa = User { data }.check().err().unwrap();
    println!("{:#}", aa);
    Ok(())
}
