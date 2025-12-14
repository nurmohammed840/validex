**🚧 In Development 🚧**

# Example

```rust
use validex::*;

#[derive(Validate)]
struct SignupData {
    #[validate(Any((
        Range(10..=20),
        Range(40..=50),
    )))]
    id: u32,
    #[validate(validate_email)]
    mail: String,
    #[validate(Length(..=20))]
    site: Option<String>,
    #[validate(Maybe(validate_unique_username))]
    first_name: Option<String>,
    #[validate(Range(18..24))]
    age: u32,
    #[validate(Range(1.0..=3.0))]
    height: f32,
}

fn validate_email<T>(_: &T) -> Result {
    Ok(())
}

fn validate_unique_username(username: &impl AsRef<str>) -> Result {
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
fn example() -> Result {
    let signup_data = SignupData {
        id: 42,
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