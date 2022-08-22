use crate::template::generator::Generator;
use crate::template::name::*;
use crate::template::text::*;
use serde_json::Value;
use std::collections::HashMap;

mod consts;
mod datetime;
mod generator;
mod id;
mod name;
mod text;
mod utils;
mod value;

pub fn gen_data<'a>(name: &str, value: &Value) -> (String, Value) {
  let gen = Generator::new();
  gen.gen_data(name, value)
}

pub fn call(name: &str, params: (usize, usize)) -> String {
  let map = get_fn_mapping();

  if let Some(f) = map.get(name) {
    return f(params.0, params.1);
  }

  "".to_string()
}

pub fn get_fn_mapping<'a>() -> HashMap<&'a str, &'a dyn Fn(usize, usize) -> String> {
  HashMap::from([
    ("name", &name as &dyn Fn(usize, usize) -> String),
    ("word", &word),
    ("sentence", &sentence),
    ("paragraph", &paragraph),
    ("uuid", &id::uuid),
  ])
}
