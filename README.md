**🚧 In Development 🚧**

# Example

```rust
use validex::{self as v, Validate};

#[derive(Validate)]
struct SignupData {
    #[validate(validate_email)]
    mail: String,
    #[validate(v::Length(..=20))]
    site: Option<String>,
    #[validate(v::Maybe(validate_unique_username))]
    first_name: Option<String>,
    #[validate(v::Range(18..24))]
    age: u32,
    #[validate(v::Range(1.0..=3.0))]
    height: f32,
}

fn validate_email<T>(_: &T) -> v::Result {
    Ok(())
}

fn validate_unique_username(username: &impl AsRef<str>) -> v::Result {
    if username.as_ref() == "xXxShad0wxXx" {
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
fn example() -> v::Result {
    let signup_data = SignupData {
        mail: "alice.smith@example.com".into(),
        site: Some("personal-blog.net".into()),
        first_name: Some("Alice".into()),
        age: 20,
        height: 1.65,
    };
    User { signup_data }.validate()?;
    Ok(())
}
```