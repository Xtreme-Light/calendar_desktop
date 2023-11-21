// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{NaiveDate, DateTime, Utc};
use thiserror::Error;

#[derive(Error, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorInfo {
    #[error("未知的异常，错误信息为 `{0}`")]
    Unknown(String),
    #[error("发生错误，错误码为 `{0}` 错误信息为 `{1}`")]
    Info(u16, String),
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn query_calendar_event_source(
    value: EventSourceRequestReq,
) -> Result<Vec<EventSrouceResp>, ErrorInfo> {
    println!("this is {:?}", value);
    if let Some(start_str) = value.start_str {
        println!("this is _start_str {:?}", start_str);
    }
    if let Some(start) = value.start {
        println!("this is start {:?}", start);
    }
    if let Some(end) = value.end {
        println!("this is end {:?}", end);
    }
    if let Some(end_str) = value.end_str {
        println!("this is end_str {:?}", end_str);
    }
    if let Some(time_zone) = value.time_zone {
        println!("this is end_str {:?}", time_zone);
    }
    let mut container: Vec<EventSrouceResp> = Vec::new();
    container.push(EventSrouceResp::new().fill_title_and_start_date("测试数据", 2023,11,20));
    container.push(EventSrouceResp::new().fill_title_and_start_date("测试数据2", 2023,11,21));
    return Ok(container);
}
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
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
    // extended_props: Identity<Dictionary>;
    #[serde(with = "my_date_format")]
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    date: Option<NaiveDate>,
    all_day: Option<bool>,
    id: Option<String>,
    group_id: Option<String>,
    title: Option<String>,
    url: Option<String>,
    interactive: Option<bool>,
}

impl EventSrouceResp {
    fn new() -> Self {
        EventSrouceResp {
            display: None,
            editable: None,
            start_editable: None,
            duration_editable: None,
            constraint: None,
            overlap: None,
            allow: None,
            class_name: None,
            class_names: None,
            color: None,
            background_color: None,
            border_color: None,
            text_color: None,
            start: None,
            end: None,
            date: None,
            all_day: None,
            id: None,
            group_id: None,
            title: None,
            url: None,
            interactive: None,
        }
    }
    fn fill_title_and_start_date(mut self,title:&str,year: i32, month: u32, day: u32) -> Self{
        self.title = Some(title.to_string());
        self.start = Some(NaiveDate::from_ymd_opt(year, month,day).unwrap());
        self
    }

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
/// https://serde.rs/custom-date-format.html
mod my_date_format {
    use chrono::{DateTime, Utc, NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}