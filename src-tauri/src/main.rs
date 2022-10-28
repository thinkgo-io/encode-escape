#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use clap::Parser;
use tauri::generate_handler;
use tauri::Builder;

mod cli;
mod constants;
mod controller;
mod log;
mod types;

use crate::cli::Arguments;
use crate::controller::*;

fn main() {
    let arguments = Arguments::parse();

    Builder::default()
        .invoke_handler(generate_handler![get_encodings, on_encode, on_log, on_log_error, on_test])
        .run(tauri::generate_context!())
        .expect("Could not launch Encode Escape");
}
