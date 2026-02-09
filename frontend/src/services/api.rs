use gloo_net::http::Request;
use nekotimer_shared::{ApiResponse, TimerConfig, ValidationError};

const BASE_URL: &str = "/api";

pub async fn fetch_timers() -> Result<Vec<TimerConfig>, String> {
    let resp = Request::get(&format!("{}/timers", BASE_URL))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let api_resp: ApiResponse<Vec<TimerConfig>> = resp.json().await.map_err(|e| e.to_string())?;

    if api_resp.success {
        Ok(api_resp.data.unwrap_or_default())
    } else {
        Err(format_errors(api_resp.errors))
    }
}

pub async fn create_timer(timer: &TimerConfig) -> Result<TimerConfig, Vec<String>> {
    let resp = Request::post(&format!("{}/timers", BASE_URL))
        .json(timer)
        .map_err(|e| vec![e.to_string()])?
        .send()
        .await
        .map_err(|e| vec![e.to_string()])?;

    let api_resp: ApiResponse<TimerConfig> =
        resp.json().await.map_err(|e| vec![e.to_string()])?;

    if api_resp.success {
        Ok(api_resp.data.unwrap())
    } else {
        Err(extract_errors(api_resp.errors))
    }
}

pub async fn update_timer(id: &str, timer: &TimerConfig) -> Result<TimerConfig, Vec<String>> {
    let resp = Request::put(&format!("{}/timers/{}", BASE_URL, id))
        .json(timer)
        .map_err(|e| vec![e.to_string()])?
        .send()
        .await
        .map_err(|e| vec![e.to_string()])?;

    let api_resp: ApiResponse<TimerConfig> =
        resp.json().await.map_err(|e| vec![e.to_string()])?;

    if api_resp.success {
        Ok(api_resp.data.unwrap())
    } else {
        Err(extract_errors(api_resp.errors))
    }
}

fn extract_errors(errors: Option<Vec<ValidationError>>) -> Vec<String> {
    errors
        .unwrap_or_default()
        .iter()
        .map(|e| e.message.clone())
        .collect()
}

fn format_errors(errors: Option<Vec<ValidationError>>) -> String {
    errors
        .unwrap_or_default()
        .iter()
        .map(|e| e.message.clone())
        .collect::<Vec<_>>()
        .join(", ")
}
