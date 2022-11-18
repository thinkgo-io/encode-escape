use shared::prelude::*;

use crate::controller::cli::show_app_info;
use crate::controller::cli::Arguments;

// Start ──────────────────────────────────────────────── //v

pub fn start_cli(arguments: &Arguments) -> ResultOk {
    if arguments.app_info {
        show_app_info();
    }
    Ok(())
}
