use rand::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

pub fn random(min: usize, max: usize) -> usize {
  let mut rng = rand::thread_rng();
  rng.gen_range(min..=max)
}

pub fn pick(arr: &Vec<Value>) -> &Value {
  let index = random(0, arr.len() - 1);

  &arr[index]
}

pub fn shuffle<T>(arr: Vec<T>, min: u32, max: u32) {}

pub fn char(pool: &str) -> char {
  let pools = HashMap::from([
    ("lower", "abcdefghijklmnopqrstuvwxyz"),
    ("upper", "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
    ("number", "0123456789"),
    ("symbol", "!@#$%^&*()[]"),
  ]);

  let chars: &[u8] = pools.get(pool).unwrap().as_bytes();
  let idx: usize = random(0, chars.len() - 1);

  chars[idx] as char
}

pub fn int(min: usize, max: usize) -> usize {
  random(min, max)
}

pub fn bool() -> bool {
  let ret = random(0, 1);
  ret == 1
}

pub fn float(min: usize, max: usize, dmin: usize, dmax: usize) -> f64 {
  let mut ret = String::new();
  let count = random(min, max);

  ret.push_str(&count.to_string());
  ret.push_str(".");
  let dcount = random(dmin, dmax);
  for _ in 0..dcount {
    ret.push(char("number"))
  }

  ret.parse().unwrap()
}

pub fn capitalize(word: &str) -> String {
  let mut chars: Vec<char> = word.chars().collect();
  let c = &mut chars.get_mut(0).unwrap();
  if c.is_ascii() {
    c.make_ascii_uppercase();
  }

  chars.into_iter().collect()
}
