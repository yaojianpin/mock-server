use crate::template;
use serde_json::json;
use serde_json::Value;

/// name|1: [ "a", "b", "c" ]
#[test]
fn array_pick() {
  let (name, value) = template::gen_data("name|1", &json!(["a", "b", "c"]));

  assert_eq!(name, "name");
  if let Value::String(v) = value {
    return assert!(["a".to_string(), "b".to_string(), "c".to_string()].contains(&v));
  }

  assert!(false);
}

/// name|3: [ { "id|+1" ["a", "b", "c"] }]
#[test]
fn array_order() {
  let (name, value) = template::gen_data("name|3", &json!([ { "id|+1": ["a", "b", "c"] }]));

  assert_eq!(name, "name");
  println!("generated: {}", value);
  if let Value::Array(arr) = value {
    assert_eq!(arr.len(), 3);

    let expects = ["a", "b", "c"];

    for (i, item) in arr.iter().enumerate() {
      let id = if let Value::String(v) = &item["id"] {
        v
      } else {
        ""
      };

      assert_eq!(id, expects[i]);
    }
  }
}

/// name|3: [ { "id|+1" 1 }]
#[test]
fn array_increment() {
  let (name, value) = template::gen_data("name|3", &json!([ { "id|+1": 1 }]));

  assert_eq!(name, "name");
  println!("generated: {}", value);
  if let Value::Array(arr) = value {
    assert_eq!(arr.len(), 3);

    for (i, item) in arr.iter().enumerate() {
      let id = if let Value::Number(v) = &item["id"] {
        v.as_i64().unwrap()
      } else {
        0
      };

      assert_eq!(id, (i + 1) as i64);
    }
  }
}
