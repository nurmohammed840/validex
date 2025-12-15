use validex::{self as v, Check};

#[derive(Check)]
struct SignupData {
    #[check(v::Any((
        v::Range(10..=20),
        v::Range(40..=50),
    )))]
    id: u32,
    #[check(check_email)]
    mail: String,
    #[check(v::Length(..=20))]
    site: Option<String>,
    #[check(v::Maybe(check_unique_username))]
    first_name: Option<String>,
    #[check(v::Range(18..24))]
    age: u32,
    #[check(v::Range(1.0..=3.0))]
    height: f32,
}

fn check_email<T>(_: &T) -> v::Result {
    Ok(())
}

fn check_unique_username(username: &impl AsRef<str>) -> v::Result {
    if username.as_ref() == "xXxShad0wxXx" {
        return Err("invalid input".into());
    }
    Ok(())
}

#[derive(Check)]
struct User {
    #[check(SignupData::check)]
    signup_data: SignupData,
}

#[test]
fn example() -> v::Result {
    let signup_data = SignupData {
        id: 42,
        mail: "alice.smith@example.com".into(),
        site: Some("personal-blog.net".into()),
        first_name: Some("Alice".into()),
        age: 20,
        height: 1.65,
    };
    User { signup_data }.check()?;
    Ok(())
}
