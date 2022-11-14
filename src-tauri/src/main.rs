mod configure;
mod controller;
mod convert;
mod data;
mod log;
mod prelude;
mod settings;
mod start;
mod types;
mod utils;

use crate::start::main::start;
use shared::prelude::*;

// Main ───────────────────────────────────────────────── //

fn main() -> ResultOk {
    start()
}
