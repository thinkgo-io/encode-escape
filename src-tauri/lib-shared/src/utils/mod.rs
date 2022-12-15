pub mod counter;
pub mod files;
pub mod first;
pub mod numbers;
pub mod rotating_counter;
pub mod strings;

use std::any::type_name;

pub use counter::Counter;
pub use files::FileMonitor;
pub use first::First;
pub use rotating_counter::RotatingCounter;

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
