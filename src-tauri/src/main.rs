#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod controller;
mod data;
mod domain;
mod log;
mod start;
mod system;

use clap::Parser;
use tauri::Builder;

use shared::prelude::*;

use crate::controller::cli::Arguments;
use crate::controller::events::*;
use crate::controller::frontend::*;
use crate::start::cli::start_cli;
use crate::system::configure::create_configuration;
use crate::system::configure::types::*;

// Main ───────────────────────────────────────────────── //

fn main() -> ResultOk {
    let configuration: ArcedConfiguration = create_configuration()?.into();
    let arguments = Arguments::parse();

    start_cli(&arguments)?;
    start_tauri(&configuration)?;
    
    Ok(())
}

pub fn start_tauri(configuration: &ArcedConfiguration) -> StandardBoxedResultOk {
    let configuration_1 = configuration.clone();
    let configuration_2 = configuration.clone();
    Builder::default()
        .setup(|app| start::tauri::setup_tauri(app, configuration_1))
        .invoke_handler(tauri::generate_handler![
            on_get_encodings,
            on_get_current_encode_operation,
            on_encode,
            on_log,
            on_log_error,
            on_test
        ])
        .on_window_event(move |event| {
            on_window_event(
                event,
                &configuration_2.runtime_settings,
                &configuration_2.runtime_settings_file,
            )
        })
        .run(tauri::generate_context!())
        .expect("Could not launch Encode Escape");
    Ok(())
}
