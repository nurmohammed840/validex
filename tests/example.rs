use validex::*;

#[derive(Check)]
struct UserData {
    #[check(Any((
        Range(10..=20),
        All((Not(45), Range(40..=50))),
        100,
    )))]
    id: u32,
    #[check(Maybe((
        Not("example.com"),
        Length(..=20),
        url,
    )))]
    site: Option<String>,
    #[check(Range(13..=28), Not(Range(18..=24)))]
    age: u32,
}

fn url(_: &impl AsRef<str>) -> Result<(), DynError<'_>> {
    // ...
    Ok(())
}

#[derive(Check)]
struct User {
    #[check(UserData::check)]
    data: UserData,
}

#[test]
fn example() {
    let user = User {
        data: UserData {
            id: 45,
            site: Some("personal-blog.net".into()),
            age: 25,
        },
    };
    if let Err(err) = user.check() {
        println!("{:}", err);
    }
}
