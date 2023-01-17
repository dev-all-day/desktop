use crate::{conf::ChatConfJson, utils};
use tauri::{utils::config::WindowUrl, window::WindowBuilder, App, GlobalShortcutManager, Manager};
use wry::application::accelerator::Accelerator;

use log::info;

pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let theme = ChatConfJson::theme();
    let chat_conf = ChatConfJson::get_chat_conf();
    let handle = app.app_handle();

    // tauri::async_runtime::spawn(async move {
    //     window::tray_window(&handle);
    // });

    if let Some(v) = chat_conf.global_shortcut {
        // info!("global_shortcut: `{}`", v);
        match v.parse::<Accelerator>() {
            Ok(_) => {
                // info!("global_shortcut_register");
                let handle = app.app_handle();
                let mut shortcut = app.global_shortcut_manager();
                shortcut
                    .register(&v, move || {
                        if let Some(w) = handle.get_window("main") {
                            if w.is_visible().unwrap() {
                                w.hide().unwrap();
                            } else {
                                w.show().unwrap();
                                w.set_focus().unwrap();
                            }
                        }
                    })
                    .unwrap_or_else(|err| {
                        info!("global_shortcut_register_error: {}", err);
                    });
            }
            Err(err) => {
                info!("global_shortcut_parse_error: {}", err);
            }
        }
    } else {
        info!("global_shortcut_unregister");
    };

    
    let app = app.handle();

    let splashscreen_window = app.get_window("splashscreen").unwrap();
    let main_window = app.get_window("main").unwrap();
    // we perform the initialization code on a new task so the app doesn't freeze
    tauri::async_runtime::spawn(async move {
        // initialize your app here instead of sleeping :)
        println!("Initializing...");
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("Done initializing.");

        // After it's done, close the splashscreen and display the main window
        splashscreen_window.close().unwrap();
        main_window.show().unwrap();
    });
        

    // auto_update
    if chat_conf.auto_update != "Disable" {
        info!("stepup::run_check_update");
        // let app = app.handle();
        utils::run_check_update(app, chat_conf.auto_update == "Silent", None);
    }

    Ok(())
}