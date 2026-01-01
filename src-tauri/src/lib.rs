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

                // Create native window menu
                use tauri::menu::{MenuBuilder, MenuItem};
                use tauri::Emitter;

                // Quit menu item with Cmd+Q accelerator on macOS
                #[cfg(target_os = "macos")]
                let quit_accelerator = Some("cmd+q");
                #[cfg(not(target_os = "macos"))]
                let quit_accelerator: Option<&str> = None;

                let quit_item = MenuItem::with_id(
                    app,
                    "quit",
                    #[cfg(target_os = "macos")]
                    "Quit Royal Kludge Configurator",
                    #[cfg(not(target_os = "macos"))]
                    "Quit",
                    true,
                    quit_accelerator,
                )?;

                // Check for updates menu item
                let check_updates_item = MenuItem::with_id(
                    app,
                    "check_updates",
                    "Check for Updates...",
                    true,
                    None::<&str>,
                )?;

                #[cfg(target_os = "macos")]
                {
                    // On macOS, add Quit to the app menu (first menu)
                    let app_menu =
                        tauri::menu::SubmenuBuilder::new(app, "Royal Kludge Configurator")
                            .item(&quit_item)
                            .build()?;

                    let help_menu = tauri::menu::SubmenuBuilder::new(app, "Help")
                        .item(&check_updates_item)
                        .build()?;

                    let menu = MenuBuilder::new(app)
                        .items(&[&app_menu, &help_menu])
                        .build()?;

                    app.set_menu(menu)?;
                }

                #[cfg(not(target_os = "macos"))]
                {
                    // On other platforms, add Quit to a File menu
                    let file_menu = tauri::menu::SubmenuBuilder::new(app, "File")
                        .item(&quit_item)
                        .build()?;

                    let help_menu = tauri::menu::SubmenuBuilder::new(app, "Help")
                        .item(&check_updates_item)
                        .build()?;

                    let menu = MenuBuilder::new(app)
                        .items(&[&file_menu, &help_menu])
                        .build()?;

                    app.set_menu(menu)?;
                }

                // Handle menu events
                app.on_menu_event(move |app_handle, event| {
                    match event.id().0.as_str() {
                        "quit" => {
                            app_handle.exit(0);
                        }
                        "check_updates" => {
                            // Emit event to frontend to trigger update check
                            app_handle.emit("menu-check-updates", ()).ok();
                        }
                        _ => {}
                    }
                });
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
