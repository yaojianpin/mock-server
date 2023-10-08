use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct RoutingRule {
    pub key: String,
    pub r#match: String,
    pub message: String,
}

#[derive(Debug, Default, serde::Deserialize, Clone)]
pub struct RoutingValue {
    pub to: String,
    pub query: Option<HashMap<String, String>>,
    pub wrapping: Option<HashMap<String, Value>>,
    pub rules: Option<Vec<RoutingRule>>,
    pub status: Option<u16>,
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
}
