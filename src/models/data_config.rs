use axum::{response::IntoResponse, Json};
use serde_json::{json, Value};
use std::collections::HashMap;

const WRAP_KEY_OK: &str = "ok";
const WRAP_KEY_ERR: &str = "err";
const WRAP_PAGE: &str = "pagination";
const WRAP_MSG: &str = "$msg";
const WRAP_DATA: &str = "$data";

const WRAP_PAGE_TOTAL: &str = "$total";
const WRAP_PAGE_PAGE: &str = "$page";
const WRAP_PAGE_SIZE: &str = "$size";
const WRAP_PAGE_ITEMS: &str = "$items";

#[derive(Debug, Default, serde::Deserialize, Clone)]
pub struct RoutingValue {
    pub to: String,
    pub query: Option<HashMap<String, String>>,
}

#[derive(Debug, Default)]
pub struct DataConfig {
    pub routing: HashMap<String, RoutingValue>,
    pub wrapping: HashMap<String, Value>,
    pub mapping: HashMap<String, String>,
}

impl DataConfig {
    pub fn new(data_config: &Value) -> DataConfig {
        let mut config = DataConfig::default();
        config.routing_parse(data_config);
        config.wrapping_parse(data_config);

        config
    }

    pub fn wrapping_result(
        &self,
        data: Result<Value, String>,
    ) -> Result<Json<Value>, impl IntoResponse> {
        match data {
            Ok(v) => self.wrap_value(WRAP_KEY_OK, "sucess", v),
            Err(e) => self.wrap_value(WRAP_KEY_ERR, &e, json!({})),
        }
    }

    pub fn wrapping_page(
        &self,
        items: Vec<&Value>,
        total: usize,
        page: usize,
        size: usize,
    ) -> Value {
        match self.wrapping.get(WRAP_PAGE) {
            Some(v) => {
                let mut obj = v.clone();

                for (key, value) in obj.as_object().unwrap().clone() {
                    if value.is_string() && value.as_str().unwrap() == WRAP_PAGE_TOTAL {
                        obj[&key] = json!(total);
                    }

                    if value.is_string() && value.as_str().unwrap() == WRAP_PAGE_PAGE {
                        obj[&key] = json!(page);
                    }

                    if value.is_string() && value.as_str().unwrap() == WRAP_PAGE_SIZE {
                        obj[&key] = json!(size);
                    }

                    if value.is_string() && value.as_str().unwrap() == WRAP_PAGE_ITEMS {
                        obj[&key] = json!(items);
                    }
                }

                obj
            }
            None => Value::Null,
        }
    }

    fn wrapping_parse(&mut self, data: &Value) {
        let wrap_data = data["wrapping"].clone();
        if !wrap_data.is_null() {
            self.wrapping = serde_json::from_value::<HashMap<String, Value>>(wrap_data).unwrap();
            //tracing::trace!("config::wrapping  {:?}", self.wrapping)
        }
    }

    fn routing_parse(&mut self, data: &Value) {
        let routing_data = data["routing"].clone();
        if !routing_data.is_null() {
            self.routing =
                serde_json::from_value::<HashMap<String, RoutingValue>>(routing_data).unwrap();
        }
    }

    fn wrap_value(&self, key: &str, msg: &str, data: Value) -> Result<Json<Value>, String> {
        match self.wrapping.get(key) {
            // find wrap data
            Some(ok) => {
                let mut obj = ok.clone();
                let mut data_key: Option<String> = None;
                let mut msg_key: Option<String> = None;
                for (key, value) in obj.as_object().unwrap().clone() {
                    if value.is_string() && value.as_str().unwrap() == WRAP_DATA {
                        data_key = Some(key.clone());
                    }
                    if value.is_string() && value.as_str().unwrap() == WRAP_MSG {
                        msg_key = Some(key.clone());
                    }
                }
                if let Some(k) = data_key {
                    obj[&k] = data;
                }
                if let Some(k) = msg_key {
                    obj[&k] = json!(msg);
                }
                Ok(Json(obj))
            }
            // not find
            None => match key {
                WRAP_KEY_OK => Ok(Json(data)),
                _ => Err(msg.to_string()),
            },
        }
    }
}
