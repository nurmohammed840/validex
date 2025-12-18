[![Crates.io](https://img.shields.io/crates/v/validex.svg)](https://crates.io/crates/validex)
[![Documentation](https://docs.rs/validex/badge.svg)](https://docs.rs/validex)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

# validex

A Rust validation library.

Unlike [validator](https://github.com/Keats/validator) library, which use syntex or string.

[validex](https://github.com/nurmohammed840/validex) use concrete rust values in `#[check(...)]` attribute.  
Any types that implement [`Check`](https://docs.rs/validex/latest/validex/trait.Check.html) trait can be used in `#[check(...)]`.

This enables IDE-friendly features like: auto-import/fix, goto-def, syntax highlight, hover docs, etc...

## Features

- `Check` derive macros for validating structs.
- Zore-cost abstractions: [`All`](https://docs.rs/validex/latest/validex/struct.All.html), [`Any`](https://docs.rs/validex/latest/validex/struct.Any.html) and [`Not`](https://docs.rs/validex/latest/validex/struct.Not.html) combinators.
- Flexible and Extensible: use functions or any type that implements `Check` trait.
- Detailed error reporting: preserves all relevant information.
- IDE friendly: Works well with Rust Analyzer.

## Example

Add `validex` to your `Cargo.toml`:

```toml
[dependencies]
validex = "0.1"
```

Simple [Parse, don’t validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/) example:

```rust,ignore
struct UserId(u32);

impl Parse for UserId {
  fn parse(buf: &mut ...) -> Result<UserId> {
    let id: u32 = buf.parse()?;
    let rule = Any((
      Range(20..=30),
      All((Not(45), Range(40..=50))),
      100,
    ));
    if !rule.verify(&id) {
      return Err(..);
    }
    Ok(UserId(id))
  }
}
```

Here is an simple example using `Check` derive macro:

```rust
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

fn main() {
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
```
