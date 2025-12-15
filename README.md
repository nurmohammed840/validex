**🚧 In Development 🚧**

# Example

```rust
use validex::*;

#[derive(Check)]
struct UserData {
    #[check(Any((
        Range(10..=20),
        30,
        Range(40..=50),
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
    #[check(Range(1.0..=3.0))]
    height: f32,
}

fn url<T: AsRef<str>>(_: &T) -> Result {
    // ...
    Ok(())
}

#[derive(Check)]
struct User {
    #[check(UserData::check)]
    data: UserData,
}

#[test]
fn example() -> Result {
    let data = UserData {
        id: 42,
        site: Some("personal-blog.net".into()),
        age: 25,
        height: 1.65,
    };
    User { data }.check()?;
    Ok(())
}
```