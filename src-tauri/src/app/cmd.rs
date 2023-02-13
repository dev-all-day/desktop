use crate::{
    // app::{fs_extra, window},
    conf::{ChatConfJson, GITHUB_PROMPTS_CSV_URL},
    utils::{self, chat_root, create_file},
};
use log::info;
use regex::Regex;
use std::{collections::HashMap, fs, path::PathBuf, vec};
use tauri::{api, command, AppHandle, Manager, Theme};
use walkdir::WalkDir;

#[command]
pub fn drag_window(app: AppHandle) {
    app.get_window("main").unwrap().start_dragging().unwrap();
}

// #[command]
// pub fn dalle2_window(app: AppHandle, query: String) {
//     window::dalle2_window(
//         &app.app_handle(),
//         Some(query),
//         Some("ChatGPT & DALL·E 2".to_string()),
//         None,
//     );
// }

#[command]
pub fn fullscreen(app: AppHandle) {
    let win = app.get_window("main").unwrap();
    if win.is_fullscreen().unwrap() {
        win.set_fullscreen(false).unwrap();
    } else {
        win.set_fullscreen(true).unwrap();
    }
}

#[command]
pub fn open_link(app: AppHandle, url: String) {
    api::shell::open(&app.shell_scope(), url, None).unwrap();
}

#[command]
pub fn get_chat_conf() -> ChatConfJson {
    ChatConfJson::get_chat_conf()
}

#[command]
pub fn get_theme() -> String {
    ChatConfJson::theme().unwrap_or(Theme::Light).to_string()
}

#[command]
pub fn reset_chat_conf() -> ChatConfJson {
    ChatConfJson::reset_chat_conf()
}

#[command]
pub fn run_check_update(app: AppHandle, silent: bool, has_msg: Option<bool>) {
    utils::run_check_update(app, silent, has_msg);
}

#[command]
pub fn form_confirm(_app: AppHandle, data: serde_json::Value) {
    ChatConfJson::amend(&serde_json::json!(data), None).unwrap();
}

#[command]
pub fn form_cancel(app: AppHandle, label: &str, title: &str, msg: &str) {
    let win = app.app_handle().get_window(label).unwrap();
    tauri::api::dialog::ask(
        app.app_handle().get_window(label).as_ref(),
        title,
        msg,
        move |is_cancel| {
            if is_cancel {
                win.close().unwrap();
            }
        },
    );
}

#[command]
pub fn form_msg(app: AppHandle, label: &str, title: &str, msg: &str) {
    let win = app.app_handle().get_window(label);
    tauri::api::dialog::message(win.as_ref(), title, msg);
}



#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PromptRecord {
    pub cmd: Option<String>,
    pub act: String,
    pub prompt: String,
}


#[command]
pub fn window_reload(app: AppHandle, label: &str) {
    app.app_handle()
        .get_window(label)
        .unwrap()
        .eval("window.location.reload()")
        .unwrap();
}
