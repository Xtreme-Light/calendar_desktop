// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::NaiveDate;
use thiserror::Error;

#[derive(Error,serde::Serialize,serde::Deserialize,Debug,PartialEq, Eq, PartialOrd, Ord)]
pub enum  ErrorInfo  {
    #[error("未知的异常，错误信息为 `{0}`")]
    Unknown(String),
    #[error("发生错误，错误码为 `{0}` 错误信息为 `{1}`")]
    Info(u16,String),
    
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn query_calendar_event_source(value: EventSourceRequestReq) -> Result<Vec<EventSrouceResp>, ErrorInfo> {
    println!("this is {:?}", value);
    if let Some(start_str) = value.start_str {
        println!("this is _start_str {:?}", start_str);
    }
    if let Some(start) = value.start{
        println!("this is start {:?}",start);
    }
    if let Some(end) = value.end {
        println!("this is end {:?}",end);
    }
    if let Some(end_str) = value.end_str {
        println!("this is end_str {:?}",end_str);
    }
    if let Some(time_zone) = value.time_zone {
        println!("this is end_str {:?}",time_zone);
    }
    return Ok(Vec::new());
}
#[derive(serde::Serialize)]
struct EventSrouceResp {
    display: Option<String>,
    editable: Option<bool>,
    start_editable: Option<bool>,
    duration_editable: Option<bool>,
    constraint: Option<String>,
    overlap: Option<bool>,
    allow: Option<String>,
    class_name: Option<String>,
    class_names: Option<String>,
    color: Option<String>,
    background_color: Option<String>,
    border_color: Option<String>,
    text_color: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct EventSourceRequestReq {
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    start_str: Option<String>,
    end_str: Option<String>,
    time_zone: Option<String>,
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, query_calendar_event_source])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
