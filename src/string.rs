use crate::*;

pub fn url(val: &str) -> Result<()> {
    let s = val.trim();
    if s.is_empty() {
        return Err("url is empty".into());
    }
    if s.chars().any(|c| c.is_whitespace()) {
        return Err("url contains whitespace".into());
    }
    Ok(())
}

pub fn email(val: &str) -> Result<()> {
    url(val.as_ref())
}

