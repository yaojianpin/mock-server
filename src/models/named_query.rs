use crate::HashMap;
use serde_json::Value;

#[derive(Default, Clone)]
pub struct NamedQuery {
    pub page: Option<u32>,
    pub size: Option<u32>,
    pub search: Option<String>,
    pub id: Option<String>,

    pub sort: Option<String>,
    pub order: Option<String>,

    query: HashMap<String, String>,
}

impl NamedQuery {
    /// convert hashmap query to named query
    pub fn from(query: &HashMap<String, String>) -> Self {
        let mut named_query = NamedQuery::default();
        named_query.query = query.clone();
        // page index
        if let Some(page) = query.get("_page") {
            named_query.page = Some(page.parse().unwrap());
            named_query.query.remove("_page");
        }

        // page size
        if let Some(size) = query.get("_size") {
            named_query.size = Some(size.parse().unwrap());
            named_query.query.remove("_size");
        }

        // common search keyword
        if let Some(search) = query.get("_q") {
            named_query.search = Some(search.clone());
            named_query.query.remove("_q");
        }

        // sort
        if let Some(sort) = query.get("_sort") {
            named_query.sort = Some(sort.clone());
            named_query.query.remove("_sort");
        }

        // order
        if let Some(order) = query.get("_order") {
            named_query.order = Some(order.clone());
            named_query.query.remove("_order");
        }

        named_query
    }

    /// get value from query by given key
    pub fn get(&self, key: &str) -> Option<&String> {
        self.query.get(key)
    }

    /// check if the query is empty
    pub fn is_empty(&self) -> bool {
        self.query.is_empty()
    }

    /// check thie item's properties matches the query
    pub fn is_match(&self, item: &Value) -> bool {
        let mut ret = true;

        // for (k, v) in &self.query {
        //   ret &= match &item[k] {
        //     Value::Bool(value) => *value == v.parse::<bool>().unwrap(),
        //     Value::Number(value) => value.as_f64().unwrap() == v.parse::<f64>().unwrap(),
        //     Value::String(value) => value == v,
        //     _ => false,
        //   };
        // }

        // continue to match the _q query
        if ret {
            let mut is_match_any = false;
            if let Some(q) = &self.search {
                for (_, v) in item.as_object().unwrap() {
                    if v.is_string() {
                        let is_match = regex::Regex::new(&q).unwrap().is_match(v.as_str().unwrap());
                        if is_match {
                            is_match_any = true;
                            break;
                        }
                    }
                }
            } else {
                // if there is no _q query,  set to true
                is_match_any = true;
            }

            ret = is_match_any;
        }

        ret
    }
}
