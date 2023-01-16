mod data_config;
mod named_query;

use std::collections::HashMap;

pub const WRAP_KEY_OK: &str = "ok";
pub const WRAP_KEY_ERR: &str = "err";
pub const WRAP_PAGE: &str = "pagination";
pub const WRAP_MSG: &str = "$msg";
pub const WRAP_DATA: &str = "$data";

pub const WRAP_PAGE_TOTAL: &str = "$total";
pub const WRAP_PAGE_PAGE: &str = "$page";
pub const WRAP_PAGE_SIZE: &str = "$size";
pub const WRAP_PAGE_ITEMS: &str = "$items";

pub use data_config::{DataConfig, RoutingRule};
pub use named_query::NamedQuery;
use serde_json::Value;
pub type Wrapper = HashMap<String, Value>;
