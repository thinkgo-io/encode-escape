#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cli;
mod configure;
mod controller;
mod data;
mod encoding;
mod log;
mod settings;

use clap::Parser;
use std::sync::Mutex;
use tauri::generate_handler;
use tauri::App;
use tauri::Builder;
use tauri::GlobalWindowEvent;
use tauri::Manager;
use tauri::Window;
use tauri::WindowEvent;

use shared::prelude::*;

use crate::cli::Arguments;
use crate::configure::create_configuration;
use crate::controller::*;
use crate::data::runtime::*;

// Init ───────────────────────────────────────────────── //

fn setup(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let window = get_main_window(app)?;

    // let configuration = create_configuration(&window.current_monitor().unwrap().unwrap())?;
    // app.manage(Mutex::new(configuration.os_settings));
    // app.manage(Mutex::new(configuration.app_settings));
    // app.manage(Mutex::new(configuration.runtime_settings));
    // app.manage(Mutex::new(configuration.runtime_settings_file));

    window.set_min_size(Some(min_window_size()))?;

    Ok(())
}

fn get_main_window(app: &App) -> Result<Window> {
    match app.get_window("main") {
        Some(window) => Ok(window),
        None => Err(Error::invalid_configuration(
            "Can't get Tauri window 'main'.",
        )),
    }
}

// Handlers ───────────────────────────────────────────── //

fn on_window_event(event: GlobalWindowEvent) {
    let window = event.window();

    match event.event() {
        WindowEvent::CloseRequested { .. } => {
            p!("Window {} - Close", window.label());
        }
        WindowEvent::Resized(physical_size) => {
            p!("Window {} - Resized  {:?}", window.label(), physical_size);
        }
        WindowEvent::Moved(physical_position) => {
            p!(
                "Window {} - Moved    {:?}",
                window.label(),
                physical_position
            );
        }
        _ => (),
    }
}

// Main ───────────────────────────────────────────────── //

fn main() {
    let arguments = Arguments::parse();

    Builder::default()
        .setup(setup)
        .invoke_handler(generate_handler![
            on_get_encodings,
            on_encode,
            on_log,
            on_log_error,
            on_set_title,
            on_test
        ])
        .on_window_event(on_window_event)
        .run(tauri::generate_context!())
        .expect("Could not launch Encode Escape");
}
