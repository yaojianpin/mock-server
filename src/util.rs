use crate::{
    db::get_config,
    models::{
        Wrapper, WRAP_DATA, WRAP_KEY_ERR, WRAP_KEY_OK, WRAP_MSG, WRAP_PAGE, WRAP_PAGE_ITEMS,
        WRAP_PAGE_PAGE, WRAP_PAGE_SIZE, WRAP_PAGE_TOTAL,
    },
    HashMap,
};
use axum::{response::IntoResponse, Json};
use serde_json::{json, Value};
use std::cmp::Ordering;

/// sort query by key
/// for updating uri, the last key should be id
// pub fn sort_keys(query: &HashMap<String, String>) -> Vec<(String, String)> {
//     tracing::debug!("sort_query_keys: query={:?}", query);
//     let query_data = query.clone();
//     let mut map = query_data.into_iter().collect::<Vec<_>>();
//     map.sort_by(|a, b| a.0.cmp(&b.0));
//     tracing::debug!("sort_query_keys: query={:?}", map);
//     map
// }

/// find json by given keys
pub fn find_by_data_name<'a>(data: &'a Value, data_name: &str) -> Result<&'a Value, String> {
    let json = &data[data_name];

    if !json.is_null() && json.is_array() {
        return Ok(&json);
    }

    Err("not found by data_name {data_name}".to_string())
}

// pub fn find_by_data_name_mut<'a>(
//     data: &'a mut Value,
//     data_name: &str,
// ) -> Result<&'a mut Value, String> {
//     let json = &mut data[data_name];

//     if !json.is_null() && json.is_array() {
//         return Ok(json);
//     }

//     Err("not found by data_name {data_name}".to_string())
// }

pub fn cmp(a: &Value, b: &Value) -> Ordering {
    if a.is_f64() {
        let a_value = a.as_f64().unwrap_or(0_f64);
        let b_value = b.as_f64().unwrap_or(0_f64);

        return if a_value > b_value {
            Ordering::Greater
        } else if a_value < b_value {
            Ordering::Less
        } else {
            Ordering::Equal
        };
    } else if a.is_i64() {
        let a_value = a.as_i64().unwrap_or(0_i64);
        let b_value = b.as_i64().unwrap_or(0_i64);
        return a_value.cmp(&b_value);
    } else if a.is_boolean() {
        let a_value = a.as_bool().unwrap_or(false);
        let b_value = b.as_bool().unwrap_or(false);
        return a_value.cmp(&b_value);
    } else if a.is_string() {
        let a_value = a.as_str().unwrap_or("");
        let b_value = b.as_str().unwrap_or("");
        return a_value.cmp(b_value);
    }

    Ordering::Equal
}

/// wrap result by config wrapping
/// `data` - data result
/// `wrapper` - wrapper config, use default wrapping config if it is none,
pub fn wrap_result(
    data: Result<Value, String>,
    wrapper: Option<Wrapper>,
) -> Result<Json<Value>, impl IntoResponse> {
    match data {
        Ok(v) => wrap_value(WRAP_KEY_OK, "success", &v, wrapper),
        Err(e) => wrap_value(WRAP_KEY_ERR, &e, &json!({}), wrapper),
    }
}

/// wrap page with config wrapping
pub fn wrap_page(
    default_wrap: &Wrapper,
    items: Vec<&Value>,
    total: usize,
    page: usize,
    size: usize,
    custom_wrap: Option<Wrapper>,
) -> Value {
    let mut wrapper = default_wrap.clone();
    if custom_wrap.is_some() {
        wrapper = custom_wrap.unwrap();
    }
    match wrapper.get(WRAP_PAGE) {
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

/// wrap value with msg and data
fn wrap_value(
    key: &str,
    msg: &str,
    data: &Value,
    custom_wrap: Option<Wrapper>,
) -> Result<Json<Value>, String> {
    let config = get_config();
    let mut wrapper = config.wrapping.clone();
    if custom_wrap.is_some() {
        wrapper = custom_wrap.unwrap();
    }

    match wrapper.get(key) {
        // find wrap data
        Some(wrap_value) => {
            let obj = replace_data(wrap_value, msg, data);
            Ok(Json(obj))
        }
        // not find
        None => match key {
            WRAP_KEY_OK => Ok(Json(data.clone())),
            _ => Err(msg.to_string()),
        },
    }
}

/// nested replace data in data object
fn replace_data(obj: &Value, msg: &str, data: &Value) -> Value {
    let mut obj = obj.clone();
    for (key, value) in obj.as_object().unwrap().clone() {
        if value.is_string() && value.as_str().unwrap() == WRAP_DATA {
            obj[key] = data.clone();
        } else if value.is_string() && value.as_str().unwrap() == WRAP_MSG {
            obj[key] = json!(msg);
        } else if value.is_object() {
            obj[key] = replace_data(&value, msg, data);
        }
    }

    obj
}
