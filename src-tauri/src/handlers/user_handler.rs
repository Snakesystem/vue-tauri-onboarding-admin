use reqwest::Client;
use tauri::State;

#[tauri::command]
pub async fn get_user_info(client: State<'_, Client>) -> Result<serde_json::Value, serde_json::Value> {
    let url = "http://localhost:8001/api/v1/user/userinfo";
    let response = client.get(url).send().await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();

            let parsed_response: Result<serde_json::Value, _> = serde_json::from_str(&text);
            match parsed_response {
                Ok(json) => {
                    if status.is_success() {
                        Ok(json)
                    } else {
                        Err(json)
                    }
                }
                Err(err) => Err(serde_json::json!({
                    "result": false,
                    "message": format!("JSON Parse Error: {}", err),
                    "error": err.to_string(),
                })),
            }
        }
        Err(err) => Err(serde_json::json!({
            "result": false,
            "message": "Network Error",
            "error": err.to_string(),
        })),
    }
}