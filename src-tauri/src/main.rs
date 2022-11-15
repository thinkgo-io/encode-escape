mod controller;
mod data;
mod domain;
mod log;
mod prelude;
mod start;
mod system;
mod utils;

use crate::start::main::start;
use shared::prelude::*;

// Main ───────────────────────────────────────────────── //

fn main() -> ResultOk {
    start()
}
