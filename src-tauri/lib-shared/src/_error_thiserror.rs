// use thiserror::Error;
// 
// pub type Result<T> = std::result::Result<T, Error>;
// pub type ResultOk = std::result::Result<(), Error>;
// 
// #[derive(Debug, Error)]
// pub enum Error {
//     #[error("Error: {cause:?}")]
//     Error {
//         #[from]
//         cause: Option<Box<dyn std::error::Error>>,
//         description: String,
//     },
//     #[error("Invalid Configuration: {description}")]
//     InvalidConfiguration { description: String },
//     #[error("Invalid Data: {description}")]
//     InvalidData { description: String },
//     #[error("Invalid Value: {description}")]
//     InvalidValue { description: String },
//     #[error("Not Found: {name}. {description}")]
//     NotFound { name: String, description: String },
//     #[error("TimeOut: {description}")]
//     TimedOut { description: String },
//     #[error("User Exit")]
//     UserExit(),
// }
// 
// impl Error {
// 
//     pub fn new(description: &str) -> Error {
//         Error::Error {
//             cause: None,
//             description: description.to_string()
//         }
//     }
// 
// }
// 
// impl From<std::io::Error> for Error {
//     fn from(cause: std::io::Error) -> Self {
//         Error::Error {
//             cause: Box::new(cause),
//         }
//     }
// }
// 
// impl From<std::num::ParseIntError> for Error {
//     fn from(cause: std::num::ParseIntError) -> Self {
//         Error::Error {
//             cause: Box::new(cause),
//         }
//     }
// }
// 
// impl From<std::string::FromUtf8Error> for Error {
//     fn from(cause: std::string::FromUtf8Error) -> Self {
//         Error::Error {
//             cause: Box::new(cause),
//         }
//     }
// }
