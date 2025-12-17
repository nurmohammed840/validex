use validex::*;

fn validate_url(_: &impl AsRef<str>) -> Result<(), String> {
    Ok(())
}

fn validate_user_id(id: &u32) -> Result<(), &'static str> {
    if *id == 13 {
        return Err("13 is an unlucky number");
    }
    Ok(())
}

#[derive(Check)]
struct UserData {
    #[check(
        Any((
            Range(20..=30),
            All((Not(45), Range(40..=50))),
            100,
        )),
        validate_user_id
    )]
    id: u32,
    #[check(Maybe((
        Not("example.com"),
        Length(..=20),
        validate_url,
    )))]
    site: Option<String>,
    #[check(Range(13..=28), Not(Range(18..=24)))]
    age: u32,
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
