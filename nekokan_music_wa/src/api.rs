use crate::types::MusicData;
use gloo_net::http::Request;
use serde_json::Value;

const API_BASE: &str = "/api";

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ListEntryWithLabel {
    pub filename: String,
    pub display_label: String,
}

#[allow(dead_code)]
pub async fn list_files() -> Result<Vec<String>, String> {
    let resp = Request::get(&format!("{}/list", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("list failed: {}", resp.status()));
    }
    let list: Vec<String> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(list)
}

pub async fn list_with_labels() -> Result<Vec<ListEntryWithLabel>, String> {
    let resp = Request::get(&format!("{}/list-with-labels", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("list-with-labels failed: {}", resp.status()));
    }
    let list: Vec<ListEntryWithLabel> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(list)
}

pub async fn get_file(name: &str) -> Result<MusicData, String> {
    let path = format!("{}/files/{}", API_BASE, name);
    let resp = Request::get(&path)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("get failed: {}", resp.status()));
    }
    let value: Value = resp.json().await.map_err(|e| e.to_string())?;
    serde_json::from_value(value).map_err(|e| e.to_string())
}

pub async fn save_file(filename: &str, data: &MusicData) -> Result<(), String> {
    let mut f = filename.trim().to_string();
    if f.ends_with(".json") {
        f = f.strip_suffix(".json").unwrap_or(&f).to_string();
    }
    let body = serde_json::json!({ "filename": f, "data": data });
    let resp = Request::post(&format!("{}/save", API_BASE))
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        let msg: Value = resp.json().await.unwrap_or(Value::Null);
        return Err(msg["error"].as_str().unwrap_or("save failed").to_string());
    }
    Ok(())
}
