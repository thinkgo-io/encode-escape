use crate::configure::create_configuration;
use crate::configure::types::*;
use crate::controller::cli::show_app_info;
use crate::controller::cli::Arguments;
use crate::start::tauri;
use clap::Parser;
use shared::prelude::*;

// Start ──────────────────────────────────────────────── //

pub fn start() -> ResultOk {
    let arguments = Arguments::parse();
    let configuration: ArcedConfiguration = create_configuration()?.into();

    if arguments.app_info {
        show_app_info();
    }

    tauri::start(&configuration)?;
    Ok(())
}
