pub use crate::error::Error;
pub use crate::types::Result;
pub use crate::types::ResultOk;
pub use crate::validate::Validator;

pub struct Wrap<T>(pub T);

pub use std::format as f;
pub use std::println as p;
