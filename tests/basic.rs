use validex::{Range, Validate};

struct SignupData {
    mail: String,
    site: String,
    first_name: String,
    age: u32,
    height: f32,
}

impl SignupData {
    fn validate(val: &Self) -> validex::Result {
        Validate::validate(&validex::email, &val.mail)?;
        Validate::validate(&validex::url, &val.site)?;
        Validate::validate(&validate_unique_username, &val.first_name)?;
        Validate::validate(&Range(18..24), &val.age)?;
        Validate::validate(&Range(0.0..=100.0), &val.height)?;
        Ok(())
    }
}

fn validate_unique_username(username: &str) -> validex::Result {
    if username == "xXxShad0wxXx" {
        return Err("invalid input".into());
    }
    Ok(())
}

#[test]
fn test_basic() -> validex::Result {
    let val = SignupData {
        mail: "alice.smith@example.com".into(),
        site: "personal-blog.net".into(),
        first_name: "Alice".into(),
        age: 20,
        height: 1.65,
    };
    Validate::validate(&SignupData::validate, &val)
}
