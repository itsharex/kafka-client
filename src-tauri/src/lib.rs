// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod core;
mod kafka;

use crate::core::{
    commands,
    config::AppConfiguration,
};

#[cfg_attr(mobile, tauri::mobile_enrty_point)]
pub fn run() {

    let mut ctx = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_theme::init(ctx.config_mut()))
        .plugin(tauri_plugin_shell::init())
        .manage(AppConfiguration::load())
        .invoke_handler(tauri::generate_handler![
            commands::get_current_cluster,
            commands::get_topics,
            commands::consume_topic_by_timestamp,
            commands::create_topic
        ])
        .run(ctx)
        .expect("error while running tauri application");

    
}
