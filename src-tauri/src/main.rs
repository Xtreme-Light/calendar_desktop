// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ::serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use thiserror::Error;
use tauri::{Manager};
#[derive(Error, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorInfo {
    #[error("未知的异常，错误信息为 `{0}`")]
    Unknown(String),
    #[error("发生错误，错误码为 `{0}` 错误信息为 `{1}`")]
    Info(u16, String),
}
#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
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
    container.push(EventSrouceResp::new().fill_title_and_start_date("测试数据", 2023, 11, 20));
    container.push(EventSrouceResp::new().fill_title_and_start_date("测试数据2", 2023, 11, 21));
    return Ok(container);
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EventSrouceResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration_editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    constraint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overlap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class_names: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_color: Option<String>,
    // extended_props: Identity<Dictionary>;
    #[serde(with = "json_date", skip_serializing_if = "Option::is_none")]
    start: Option<NaiveDate>,
    #[serde(with = "json_date", skip_serializing_if = "Option::is_none")]
    end: Option<NaiveDate>,
    #[serde(with = "json_date", skip_serializing_if = "Option::is_none")]
    date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all_day: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    fn fill_title_and_start_date(mut self, title: &str, year: i32, month: u32, day: u32) -> Self {
        self.title = Some(title.to_string());
        self.start = Some(NaiveDate::from_ymd_opt(year, month, day).unwrap());
        self
    }
}

#[derive(Deserialize, Debug)]
struct EventSourceRequestReq {
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    start_str: Option<String>,
    end_str: Option<String>,
    time_zone: Option<String>,
}
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .invoke_handler(tauri::generate_handler![greet, query_calendar_event_source])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
/// https://serde.rs/custom-date-format.html
mod json_date {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return match date {
            Some(date) => {
                let s = format!("{}", date.format(FORMAT));
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_str("null"),
        };
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.eq("null") {
            return Ok(None);
        }
        let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(Some(dt))
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    first_name: String,
    last_name: String,
}
mod test {

    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};

    use crate::json_date;
    #[test]
    fn test() {
        let json_str = r#"
      {
        "timestamp": "2017-02-16",
        "bidder": "Skrillex"
      }
    "#;

        let data: StructWithCustomDate = serde_json::from_str(json_str).unwrap();
        println!("{:#?}", data);

        let serialized = serde_json::to_string_pretty(&data).unwrap();
        println!("{}", serialized);
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct StructWithCustomDate {
        // DateTime supports Serde out of the box, but uses RFC3339 format. Provide
        // some custom logic to make it use our desired format.
        #[serde(default, with = "json_date")]
        pub timestamp: Option<NaiveDate>,

        // Any other fields in the struct.
        pub bidder: String,
    }
}
