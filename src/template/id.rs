use uuid::Uuid;

pub fn uuid(_min: usize, _max: usize) -> String {
  Uuid::new_v4().to_string()
}
