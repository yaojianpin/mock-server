use crate::template;
use serde_json::json;
use serde_json::Value;

/// name: { "name": "@name" }
#[test]
fn fun_name() {
    let (name, value) = template::gen_data("name", &json!("@name"));

    assert_eq!(name, "name");
    if let Value::String(v) = value {
        assert!(v.contains(" "))
    }
}

#[test]
fn fun_word() {
    let (name, value) = template::gen_data("name", &json!("@word"));

    println!("{}", value);
    assert_eq!(name, "name");
    if let Value::String(v) = value {
        return assert!(v.len() > 0);
    }
    assert!(false);
}

#[test]
fn fun_word_with_one_param() {
    let (name, value) = template::gen_data("name", &json!("@word(10)"));

    println!("{}", value);
    assert_eq!(name, "name");
    if let Value::String(v) = value {
        return assert!(v.len() == 10);
    }
    assert!(false);
}

#[test]
fn fun_word_with_min_max_params() {
    let (name, value) = template::gen_data("name", &json!("@word(5, 10)"));

    println!("{}", value);
    assert_eq!(name, "name");
    if let Value::String(v) = value {
        return assert!(v.len() >= 5 && v.len() <= 10);
    }
    assert!(false);
}

#[test]
fn fun_sentence() {
    let (name, value) = template::gen_data("name", &json!("@sentence"));

    println!("{}", value);
    assert_eq!(name, "name");
    if let Value::String(v) = value {
        return assert!(v.contains(".") && v.contains(" "));
    }
    assert!(false);
}

#[test]
fn fun_sentence_with_one_param() {
    let (name, value) = template::gen_data("name", &json!("@sentence(4)"));

    println!("{}", value);
    assert_eq!(name, "name");
    if let Value::String(v) = value {
        assert!(v.contains("."));

        let arr: Vec<&str> = v.split(" ").collect();
        println!("{:?}", arr);
        assert!(arr.len() == 4);
    }
}

#[test]
fn fun_sentence_with_min_max_params() {
    let (name, value) = template::gen_data("name", &json!("@sentence(4, 10)"));

    println!("{}", value);
    assert_eq!(name, "name");
    if let Value::String(v) = value {
        assert!(v.contains("."));

        let arr: Vec<&str> = v.split(" ").collect();
        println!("{:?}", arr);
        assert!(arr.len() >= 4 && arr.len() <= 10);
    }
}

#[test]
fn fun_paragraph() {
    let (name, value) = template::gen_data("name", &json!("@paragraph"));

    println!("{}", value);
    assert_eq!(name, "name");
    if let Value::String(v) = value {
        let arr: Vec<&str> = v.split(".").collect();
        println!("{:?}", arr);
        assert!(arr.len() > 0);
    }
}

#[test]
fn fun_paragraph_with_one_param() {
    let (name, value) = template::gen_data("name", &json!("@paragraph(5)"));

    println!("{}", value);
    assert_eq!(name, "name");
    if let Value::String(v) = value {
        let arr: Vec<&str> = v.split(".").filter(|item| *item != "").collect();
        println!("{:?}", arr);
        assert!(arr.len() == 5);
    }
}

#[test]
fn fun_paragraph_with_min_max_params() {
    let (name, value) = template::gen_data("name", &json!("@paragraph(5, 10)"));

    println!("{}", value);
    assert_eq!(name, "name");
    if let Value::String(v) = value {
        let arr: Vec<&str> = v.split(".").filter(|item| *item != "").collect();
        println!("{:?}", arr);
        assert!(arr.len() >= 5 && arr.len() <= 10);
    }
}
