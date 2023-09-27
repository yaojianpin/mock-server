use crate::io::BufReader;
use crate::models::DataConfig;
use crate::models::NamedQuery;
use crate::template::gen_data;
use crate::util;
use once_cell::sync::OnceCell;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use tracing::error;

#[derive(Clone)]
pub struct Database {
    collections: Arc<RwLock<HashMap<String, Value>>>,
    files: HashMap<String, String>,
    config: Arc<DataConfig>,
}

// global data config
pub static DATA_CONFIG: OnceCell<Arc<DataConfig>> = OnceCell::new();

pub fn get_config() -> Arc<DataConfig> {
    DATA_CONFIG.get().unwrap().clone()
}

impl<'a> Database {
    pub fn new() -> Self {
        Database {
            config: Arc::new(DataConfig::default()),
            collections: Arc::new(RwLock::new(HashMap::new())),
            files: HashMap::new(),
        }
    }

    /// init db data
    pub fn init(&mut self, path: &str) {
        let file = std::fs::File::open(path);
        if let Ok(f) = file {
            let reader = BufReader::new(f);
            let result: std::result::Result<Value, _> = serde_json::from_reader(reader);

            if let Ok(json) = result {
                // store the data
                let mut collections = self.collections.write().unwrap();
                let config = json.get("config").unwrap();
                let data_list = json.get("data").unwrap().as_object().unwrap();
                for (key, value) in data_list {
                    let (name, data) = gen_data(key, value);
                    collections.insert(name, data);
                }

                if let Some(file) = json.get("file") {
                    if let Some(file_list) = file.as_object() {
                        for (key, value) in file_list {
                            if let Some(path) = value.as_str() {
                                self.files.insert(key.to_string(), path.to_string());
                            } else {
                                error!("file.{} must be string type", key);
                            }
                        }
                    }
                }

                self.config = Arc::new(DataConfig::new(&config));
                DATA_CONFIG.set(self.config.clone()).unwrap();
            }
        }
    }

    pub fn get_config(&self) -> &DataConfig {
        &self.config
    }

    pub fn query_data(
        &self,
        path_map: &HashMap<String, String>,
        query: &HashMap<String, String>,
    ) -> Result<Value, String> {
        tracing::debug!("query_data: path_map={:?}, query={:?}", path_map, query);
        let data = self.collections.read().unwrap();

        let data_name = path_map.get("data").unwrap();
        let q = NamedQuery::from(&query);

        let json_option = data.get(data_name);
        if json_option == None {
            return Err(format!("not found data by name '{}'", data_name));
        }
        let json = json_option.unwrap();

        if !json.is_array() {
            return Ok(json.clone());
        }

        // query data by input parameters
        let arr = json.as_array().unwrap();
        let mut list: Vec<Value> = Vec::new();

        for item in arr {
            if q.is_match(item) {
                list.push(item.clone())
            }
        }

        if let Some(sort) = q.sort {
            let mut is_desc = false;

            if let Some(order) = q.order {
                if order == "desc" {
                    is_desc = true;
                }
            }

            list.sort_by(|a, b| {
                let a_value = &a[&sort];
                let b_value = &b[&sort];

                if is_desc {
                    return util::cmp(b_value, a_value);
                }

                util::cmp(a_value, b_value)
            })
        }

        // pagination
        if q.page.is_some() {
            // default page size to 10
            let size = q.size.unwrap_or(10) as usize;
            let page = q.page.unwrap() as usize;

            let total = list.len();
            let items: Vec<&Value> = list.iter().skip((page - 1) * size).take(size).collect();

            let config = self.get_config();
            let page_data = util::wrap_page(&config.wrapping, items, total, page, size, None);
            return Ok(page_data);
        }

        Ok(Value::Array(list))
    }

    pub fn get_data(&self, path_map: &HashMap<String, String>) -> Result<Value, String> {
        tracing::debug!("get_data: path_map={:?}", path_map);
        let collections = self.collections.read().unwrap();

        let data_name = path_map.get("data").unwrap();
        let id = path_map.get("id").unwrap();
        let res_data = &collections[data_name];

        if !res_data.is_null() && res_data.is_array() {
            // query data by input parameters
            let arr: Vec<_> = res_data
                .as_array()
                .unwrap()
                .iter()
                .filter(|item| &item["id"].to_string() == id)
                .map(|item| item.clone())
                .collect();

            if arr.len() == 1 {
                Ok(arr[0].clone())
            } else if arr.len() > 1 {
                Err(format!("found multipe records by id {}", id))
            } else {
                Err(format!("not found data by id {}", id))
            }
        } else {
            Err(format!("not found data by id {}", id))
        }
    }

    pub fn create_data(
        &mut self,
        path_map: &HashMap<String, String>,
        value: Value,
    ) -> Result<Value, String> {
        tracing::debug!("create_data path_map={:?}, data={:?}", path_map, value);
        let collections = &mut self.collections.write().unwrap();

        let data_name = path_map.get("data").unwrap();
        let json = collections.get_mut(data_name).unwrap();

        let list = json.as_array_mut().unwrap();
        list.push(value.clone());

        Ok(value)
    }

    pub fn update_data(
        &mut self,
        path_map: &HashMap<std::string::String, std::string::String>,
        value: Value,
    ) -> Result<Value, String> {
        tracing::debug!("update_data path_map={:?}, data={:?}", path_map, value);
        let collections = &mut self.collections.write().unwrap();

        let data_name = path_map.get("data").unwrap();
        let id = path_map.get("id").unwrap();

        let json = collections.get_mut(data_name).unwrap();

        let list = json.as_array_mut().unwrap();
        let found_list: Vec<_> = list
            .into_iter()
            .filter(|item| &item["id"].to_string() == id)
            .collect();
        let len = found_list.len();

        // check if the item exists
        if len == 0 {
            return Err(format!("not found item by id {id}"));
        }

        // modify the item data
        found_list.into_iter().for_each(|item: &mut Value| {
            *item = value.clone();
        });

        Ok(value)
    }

    pub fn delete_data(
        &mut self,
        path_map: &HashMap<std::string::String, std::string::String>,
    ) -> Result<Value, String> {
        tracing::debug!("delete_data path_map={:?}", path_map);
        let collections = &mut self.collections.write().unwrap();

        let data_name = path_map.get("data").unwrap();
        let id = path_map.get("id").unwrap();
        let json = collections.get_mut(data_name).unwrap();

        let list = json.as_array_mut().unwrap();
        let found_index: Vec<_> = list
            .into_iter()
            .enumerate()
            .filter(|(_, item)| &item["id"].to_string() == id)
            .map(|(index, _)| index)
            .collect();
        let len = found_index.len();

        // check if the item exists
        if len == 0 {
            return Err(format!("not found item by id {id}"));
        }

        for index in found_index {
            list.remove(index);
        }

        Ok(Value::Bool(true))
    }

    pub fn get_file(
        &mut self,
        path_map: &HashMap<std::string::String, std::string::String>,
    ) -> Result<String, String> {
        let file_id = path_map.get("id").unwrap();
        match self.files.get(file_id) {
            Some(v) => Ok(v.clone()),
            None => Err(format!("not found item by id {}", file_id)),
        }
    }
}
