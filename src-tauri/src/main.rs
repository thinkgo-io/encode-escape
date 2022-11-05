#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod cli;
mod controller;
mod encoding;
mod log;

use clap::Parser;
use tauri::generate_handler;
use tauri::Builder;

use cli::Arguments;
use controller::*;

fn main() {
    let arguments = Arguments::parse();

    Builder::default()
        .invoke_handler(generate_handler![
            on_get_encodings,
            on_encode,
            on_log,
            on_log_error,
            on_set_title,
            on_test
        ])
        .run(tauri::generate_context!())
        .expect("Could not launch Encode Escape");
}
