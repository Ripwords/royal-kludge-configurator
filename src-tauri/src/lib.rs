mod commands;
mod hid;
mod keyboard;
mod models;
mod modes;
mod protocol;

use commands::init_hid_manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize HID manager
    let hid_manager = init_hid_manager().expect("Failed to initialize HID manager");

    let mut builder = tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                app.handle()
                    .plugin(tauri_plugin_updater::Builder::new().build())
                    .expect("Failed to initialize updater plugin");
            }
            Ok(())
        })
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_process::init());

    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(tauri_plugin_macos_permissions::init());
    }

    builder
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(
                    "sqlite:rk_configurator.db",
                    vec![
                        tauri_plugin_sql::Migration {
                            version: 1,
                            description: "create initial tables",
                            sql: include_str!("../migrations/001_initial.sql"),
                            kind: tauri_plugin_sql::MigrationKind::Up,
                        },
                        tauri_plugin_sql::Migration {
                            version: 2,
                            description: "add selected_profile_id to keyboard_configs",
                            sql: include_str!("../migrations/002_add_selected_profile_id.sql"),
                            kind: tauri_plugin_sql::MigrationKind::Up,
                        },
                    ],
                )
                .build(),
        )
        .manage(hid_manager)
        .invoke_handler(tauri::generate_handler![
            commands::scan_keyboards,
            commands::send_keyboard_config,
            commands::get_lighting_modes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
