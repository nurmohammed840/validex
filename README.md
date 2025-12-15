**🚧 In Development 🚧**

# Example

```rust
use validex::*;

#[derive(Check)]
struct SignupData {
    #[check(Any((
        Range(10..=20),
        Range(40..=50),
    )))]
    id: u32,
    #[check(email)]
    mail: String,
    #[check(Length(..=20))]
    site: Option<String>,
    #[check(Maybe(unique_username))]
    first_name: Option<String>,
    #[check(Range(18..24))]
    age: u32,
    #[check(Range(1.0..=3.0))]
    height: f32,
}

fn email<T>(_: &T) -> Result {
    Ok(())
}

fn unique_username(username: &impl AsRef<str>) -> Result {
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
fn example() -> Result {
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
```