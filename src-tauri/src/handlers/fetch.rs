use reqwest::Client;
use tauri::State;
use crate::models::auth_model::{ActionResult, LoginRequest};

#[tauri::command]
pub async fn check_session(client: State<'_, Client>) -> Result<ActionResult, ActionResult> {
    let url = "http://localhost:8000/api/v1/auth/session";
    let response = client.get(url).send().await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            println!("API Response: {}", text);

            // Coba parse response JSON apa pun statusnya
            let parsed_response: Result<ActionResult, _> = serde_json::from_str(&text);

            match parsed_response {
                Ok(json) => {
                    if status.is_success() {
                        Ok(json)
                    } else {
                        Err(json) // Error tetap dalam format JSON dari API
                    }
                }
                Err(err) => Err(ActionResult {
                    result: false,
                    message: err.to_string(),
                    data: None,
                    error: Some(err.to_string()),
                }),
            }
        }
        Err(err) => Err(ActionResult {
            result: false,
            message: "Network Error".to_string(),
            data: None,
            error: Some(err.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn login( client: State<'_, Client>, payload: LoginRequest) -> Result<ActionResult, ActionResult> {
    let url = "http://localhost:8000/api/v1/auth/login";

    let response: Result<reqwest::Response, reqwest::Error> = client.post(url).json(&payload).send().await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();

            let parsed_response: Result<ActionResult, _> = serde_json::from_str(&text);

            match parsed_response {
                Ok(json) => {
                    if status.is_success() {
                        Ok(json)
                    } else {
                        Err(json)
                    }
                }
                Err(err) => Err(ActionResult {
                    result: false,
                    message: err.to_string(),
                    data: None,
                    error: Some(err.to_string()),
                }),
            }
        }
        Err(err) => Err(ActionResult {
            result: false,
            message: "Network Error".to_string(),
            data: None,
            error: Some(err.to_string()),
        }),
    }
}