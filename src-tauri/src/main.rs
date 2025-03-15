// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod models {
    pub mod auth_model;
}

fn main() {
    vue_tauri_onboarding_admin_lib::run()
}
