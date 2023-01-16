use crate::template;
use serde_json::json;
use serde_json::Value;

/// name: { "test|3-10": 6 }
#[test]
fn object_gen() {
    let (name, value) = template::gen_data("name", &json!({ "test|3-10": 6 }));

    assert_eq!(name, "name");

    if let Value::Object(v) = value {
        if let Value::Number(number) = &v["test"] {
            let num = number.as_i64().unwrap();
            return assert!(num >= 3 && num <= 10);
        }
    }

    assert!(false);
}
