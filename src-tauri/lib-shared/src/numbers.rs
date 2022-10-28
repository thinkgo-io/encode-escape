use crate::ResultsIn;

pub fn to_i32(text: &str) -> ResultsIn<i32> {
    let trimmed = text.trim();
    Ok(trimmed.parse::<i32>()?)
}
