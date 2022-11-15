mod controller;
mod data;
mod domain;
mod log;
mod start;
mod system;

use crate::start::main::start;
use shared::prelude::*;

// Main ───────────────────────────────────────────────── //

fn main() -> ResultOk {
    start()
}
