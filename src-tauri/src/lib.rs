// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod core;
mod kafka;

use crate::core::{
    commands,
    config::AppConfiguration,
};
use regex::Regex;
use tauri::{
    menu::{ MenuBuilder, MenuItem, MenuItemBuilder, SubmenuBuilder }, 
    LogicalPosition, 
    Manager, 
    State, 
    Wry
};

#[cfg_attr(mobile, tauri::mobile_enrty_point)]
pub fn run() {

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppConfiguration::load())
        .invoke_handler(tauri::generate_handler![
            commands::get_current_cluster,
            commands::get_topics,
            commands::consume_topic_by_timestamp,
            commands::create_topic
        ])
        .setup(|app| {
            let mut menu_items = app.state::<AppConfiguration>().config
                .lock()
                .unwrap()
                .clusters()
                .into_iter()
                .map(|item| {
                    let id = format!("cluster_item_{}", item.name);
                    let title = format!("{} ({})", item.name, item.bootstrap_servers.join(","));
                    let item = MenuItemBuilder::with_id(id, title).build(app).unwrap();
                    item
                })
                .take(3)
                .collect::<Vec<MenuItem<Wry>>>();
            let item1 = menu_items.pop();
            let item2 = menu_items.pop();
            let item3 = menu_items.pop();

            let mut submenu_builder = SubmenuBuilder::new(app, "Clusters");
            if let Some(menu) = item1 {
                submenu_builder = submenu_builder.item(&menu);
            }
            if let Some(menu) = item2 {
                submenu_builder = submenu_builder.item(&menu);
            }
            if let Some(menu) = item3 {
                submenu_builder = submenu_builder.item(&menu);
            }
            let clusters_submenu = submenu_builder.build().unwrap();
            
            app.set_menu(MenuBuilder::new(app).item(&clusters_submenu).build().unwrap());
            app.on_menu_event(|app, event| {
                let state: State<AppConfiguration> = app.state();
                let regex = Regex::new(r"cluster_item_(.+)").unwrap();
                if let Some(captures) = regex.captures(event.id().0.as_str()) {
                    if let Some(cluster_name) = captures.get(1) {
                        let updated_cluster_config = state
                            .config
                            .lock()
                            .unwrap()
                            .set_default_cluster(cluster_name.as_str())
                            .unwrap();
                        app.emit("current-cluster-update", updated_cluster_config)
                            .unwrap();
                    }
                }
            });

            let window = app.get_webview_window("main").unwrap();
            window
                .set_position(LogicalPosition::new(1200, 10))
                .expect("Error while chainging position of the window!");
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    
}
