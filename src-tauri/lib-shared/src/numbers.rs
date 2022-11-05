use crate::error::Result;

pub fn to_i32(text: &str) -> Result<i32> {
    let trimmed = text.trim();
    Ok(trimmed.parse::<i32>()?)
}
