use crate::template::utils;

pub fn word(min: usize, max: usize) -> String {
  let len = utils::random(min, max);
  let mut result = String::new();
  for _ in 0..len {
    result.push(utils::char("lower"))
  }

  result
}

pub fn sentence(min: usize, max: usize) -> String {
  let len = utils::random(min, max);
  let mut ret: Vec<String> = Vec::new();
  for _ in 0..len {
    ret.push(word(3, 10))
  }

  ret[0] = utils::capitalize(&ret[0]);
  ret.join(" ") + "."
}

pub fn paragraph(min: usize, max: usize) -> String {
  let len = utils::random(min, max);
  let mut ret: Vec<String> = Vec::new();
  for _ in 0..len {
    ret.push(sentence(12, 18))
  }
  return ret.join(" ");
}
