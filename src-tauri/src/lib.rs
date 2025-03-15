use std::sync::Arc;
use reqwest_cookie_store::{CookieStoreMutex, CookieStore};
use reqwest::{Client, ClientBuilder};
use tauri::Manager as _;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod handlers {
    pub mod auth_handler; // Import langsung tanpa mod.rs
    pub mod user_handler; // Import langsung tanpa mod.rs
}

mod models {
    pub mod auth_model;
    pub mod user_model;
}

// Function buat bikin HTTP client dengan cookie store
fn create_http_client() -> Client {
    let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::default()));

    ClientBuilder::new()
        .cookie_provider(cookie_store) // Simpan cookies otomatis
        .build()
        .expect("Failed to build client")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let client = create_http_client();
            app.manage(client); // Simpan Client di State Tauri
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            // handlers untuk authentikasi,
            handlers::auth_handler::check_session,
            handlers::auth_handler::login,
            handlers::auth_handler::logout,

            // handlers untuk user
            handlers::user_handler::get_user_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
