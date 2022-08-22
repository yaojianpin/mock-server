use crate::template;
use serde_json::json;
use serde_json::Value;

/// name: "abc"
#[test]
fn none_template_str() {
  let (name, value) = template::gen_data("name", &json!("abc"));

  assert_eq!(name, "name");
  assert!(matches!(value, Value::String(v) if v == "abc" ));
}

/// name:  123
#[test]
fn none_template_number() {
  let (name, value) = template::gen_data("name", &json!(123));

  assert_eq!(name, "name");
  assert!(matches!(value, Value::Number(v) if v.as_i64().unwrap() == 123  ));
}

/// name:  {}
#[test]
fn none_template_object() {
  let (name, value) = template::gen_data("name", &json!({}));

  assert_eq!(name, "name");
  assert!(matches!(value, Value::Object(v) if v.len() == 0  ));
}

/// name:  []
#[test]
fn none_template_arr() {
  let (name, value) = template::gen_data("name", &json!([]));

  assert_eq!(name, "name");
  assert!(matches!(value, Value::Array(v) if v.len() == 0  ));
}

/// name|min-max: "abc"
#[test]
fn random_str_count() {
  let (name, value) = template::gen_data("name|1-20", &json!("abc"));

  assert_eq!(name, "name");
  assert!(matches!(value, Value::String(v) if v.len() >= 3 && v.len() <= 20*3 ));
}

/// name|3: "a"
#[test]
fn fixed_str_count() {
  let (name, value) = template::gen_data("name|3", &json!("a"));

  assert_eq!(name, "name");
  assert!(matches!(value, Value::String(v) if v.len()==3 ));
}

/// name|min-max: [{}]
#[test]
fn random_count_array() {
  let (name, value) = template::gen_data("name|1-20", &json!(["abc"]));

  assert_eq!(name, "name");
  assert!(matches!(value, Value::Array(v) if v.len() >= 1 && v.len() <= 20 ));
}

/// name|count: [{}]
#[test]
fn fixed_count_array() {
  let (name, value) = template::gen_data("name|3", &json!(["a"]));

  assert_eq!(name, "name");
  assert!(matches!(value, Value::Array(v) if v.len()==3 ));
}

/// name|+1: [{},{},{}]
#[test]
fn serial_count_str() {
  let (name, value) = template::gen_data("name|+1", &json!(["a", "b", "c"]));

  assert_eq!(name, "name");
  assert!(matches!(value, Value::String(v) if v == "a" ));
}

/// name|1-100: 1
#[test]
fn random_number_u64() {
  let (name, value) = template::gen_data("name|1-100", &json!(1));

  assert_eq!(name, "name");
  assert!(
    matches!(value, Value::Number(v) if v.as_u64().unwrap() >= 1 && v.as_u64().unwrap() <= 100 )
  );
}

/// name|1-100.3-10: 10.123
#[test]
fn random_decimal_number_f64() {
  let (name, value) = template::gen_data("name|1-100.3-10", &json!(10.123));

  assert_eq!(name, "name");
  if let Value::Number(v) = value {
    let s = v.to_string();
    let arr: Vec<&str> = s.split(".").collect();

    let integer: i64 = arr[0].parse().unwrap();
    let decimal_str = arr[1];

    assert!(integer >= 1 && integer <= 100);
    assert!(decimal_str.len() >= 3 && decimal_str.len() <= 10);
  } else {
    assert!(false)
  }
}

/// name|1-100.123: 10.123
#[test]
fn min_decimal_number_f64() {
  let (name, value) = template::gen_data("name|1-100.4", &json!(10.123));

  println!("{}", value);
  assert_eq!(name, "name");
  if let Value::Number(v) = value {
    let s = v.to_string();
    let arr: Vec<&str> = s.split(".").collect();

    let integer: i64 = arr[0].parse().unwrap();
    let decimal_str = arr[1];

    assert!(integer >= 1 && integer <= 100);
    assert!(decimal_str.len() == 4);
  } else {
    assert!(false)
  }
}

/// name|1-100.123: 10.123
#[test]
fn dot_decimal_number_f64() {
  let (name, value) = template::gen_data("name|.3-10", &json!(0.123));

  assert_eq!(name, "name");
  if let Value::Number(v) = value {
    let s = v.to_string();
    let arr: Vec<&str> = s.split(".").collect();

    let integer: i64 = arr[0].parse().unwrap();
    let decimal_str = arr[1];
    assert!(integer == 0);
    assert!(decimal_str.len() >= 3 && decimal_str.len() <= 10);
  } else {
    assert!(false)
  }
}
