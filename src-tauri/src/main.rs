// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use async_openai::{
//     types::{CreateImageRequestArgs, ImageSize, ResponseFormat},
//     Client,
// };
// use std::error::Error;

use std::env;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn check_api_key(key_var_name: &str) -> bool {
    env::var(key_var_name).is_ok()
}

#[tauri::command]
fn set_api_key(key_var_name: &str, key: &str) {
    env::set_var(key_var_name, key);
    println!("{}: {}", key_var_name, env::var(key_var_name).unwrap());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_api_key, set_api_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
