use validex::*;

#[derive(Validate)]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(url)]
    site: String,
    #[validate(validate_unique_username)]
    first_name: String,
    #[validate(Range(18..24))]
    age: u32,
    #[validate(Range(1.0..=3.0))]
    height: f32,
}

fn validate_unique_username(username: &str) -> validex::Result {
    if username == "xXxShad0wxXx" {
        return Err("invalid input".into());
    }
    Ok(())
}

#[derive(Validate)]
struct User {
    #[validate(SignupData::validate)]
    signup_data: SignupData,
}

#[test]
fn test_basic() -> validex::Result {
    let signup_data = SignupData {
        mail: "alice.smith@example.com".into(),
        site: "personal-blog.net".into(),
        first_name: "Alice".into(),
        age: 20,
        height: 1.65,
    };

    User { signup_data }.validate()
}
