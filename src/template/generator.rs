use crate::template::call;
use crate::template::utils;
use regex::Regex;
use serde_json::json;
use serde_json::Map;
use serde_json::Number;
use serde_json::Value;

pub struct Generator;

#[derive(Default, Clone)]
pub struct GeneratorContext {
  inc: usize,
  order_index: usize,
}

#[derive(Default, Debug, Clone)]
pub struct Rule {
  pub name: String,
  pub is_rule: bool,

  pub is_range: bool,
  pub count: Option<usize>,
  pub min: Option<usize>,
  pub max: Option<usize>,

  pub is_step: bool,
  pub step: Option<usize>,

  pub is_drange: bool,
  pub dmin: Option<usize>,
  pub dmax: Option<usize>,
  pub dcount: Option<usize>,
}

impl Generator {
  pub fn new() -> Self {
    Self
  }

  /// generate data from name and value
  pub fn gen_data(&self, name: &str, value: &Value) -> (String, Value) {
    let mut context = GeneratorContext::default();
    self.gen_data_with_context(name, value, &mut context)
  }

  fn gen_data_with_context(
    &self,
    name: &str,
    value: &Value,
    context: &mut GeneratorContext,
  ) -> (String, Value) {
    let rule = self.parse_rule(name);
    let data = match value {
      Value::Array(v) => self.array(&rule, v, context),
      Value::Object(v) => self.object(&rule, v, context),
      Value::Number(v) => self.number(&rule, v, context),
      Value::String(v) => self.string(&rule, v, context),
      Value::Bool(v) => self.bool(&rule, v, context),
      _ => value.clone(),
    };

    (rule.name, data)
  }

  /// name rule
  /// regex from mockjs RE_KEY git://github.com/nuysoft/Mock.git
  ///
  /// name|min-max': value
  /// name|count': value
  /// name|min-max.dmin-dmax': value
  /// name|min-max.dcount': value
  /// name|count.dmin-dmax': value
  /// name|count.dcount': value
  /// name|+step': value
  /// 1 name, 2 step, 3 range [ min, max ], 4 drange [ dmin, dmax ]
  pub fn parse_rule<'a>(&self, name: &'a str) -> Rule {
    let re_name =
      Regex::new(r"(.+)\|(?:\+(\d+)|([\+\-]?\d+-?[\+\-]?\d*)?(?:\.(\d+-?\d*))?)").unwrap();
    let re_range = Regex::new(r"([\+\-]?\d+)-?([\+\-]?\d+)?").unwrap();
    let capture = re_name.captures(name);
    let mut rule = Rule::default();

    rule.name = name.to_string();
    if let Some(cap) = capture {
      // real name
      rule.name = cap.get(1).map(|m| m.as_str()).unwrap().to_string();

      rule.is_rule = true;

      // step
      if let Some(step) = cap.get(2) {
        rule.is_step = true;
        rule.step = Some(step.as_str().parse::<usize>().unwrap());
      }
      // range
      if let Some(range) = cap.get(3) {
        let range_capture = re_range.captures(range.as_str());
        if let Some(range) = range_capture {
          rule.is_range = true;
          // min
          if let Some(min) = range.get(1) {
            rule.min = Some(min.as_str().parse::<usize>().unwrap());
          }
          // max
          if let Some(max) = range.get(2) {
            rule.max = Some(max.as_str().parse::<usize>().unwrap());
          }
          // caculate count
          if rule.min.is_some() && rule.max.is_some() {
            rule.count = Some(utils::random(rule.min.unwrap(), rule.max.unwrap()));
          } else if rule.min.is_some() {
            rule.count = rule.min;
          }
        }
      }
      // drange
      if let Some(drange) = cap.get(4) {
        let range_capture = re_range.captures(drange.as_str());
        if let Some(range) = range_capture {
          rule.is_drange = true;
          // dmin
          if let Some(dmin) = range.get(1) {
            rule.dmin = Some(dmin.as_str().parse::<usize>().unwrap());
          }
          // dmax
          if let Some(dmax) = range.get(2) {
            rule.dmax = Some(dmax.as_str().parse::<usize>().unwrap());
          }
          // caculate dcount
          if rule.dmin.is_some() && rule.dmax.is_some() {
            rule.dcount = Some(utils::random(rule.dmin.unwrap(), rule.dmax.unwrap()));
          } else if rule.dmin.is_some() {
            rule.dcount = rule.dmin;
          }
        }
      }
    }
    rule
  }

  fn array(&self, rule: &Rule, data: &Vec<Value>, context: &mut GeneratorContext) -> Value {
    let mut result: Vec<Value> = Vec::new();

    if !rule.is_rule {
      let mut context = context.clone();
      for (i, v) in data.iter().enumerate() {
        context.inc = i;
        let (_, d) = self.gen_data_with_context(&i.to_string(), v, &mut context);
        result.push(d);
      }
    } else {
      if rule.is_range && rule.min.unwrap() == 1 && rule.max.is_none() {
        // just return value, not the array type
        return json!(utils::pick(data).clone());
      } else if rule.is_step {
        let index = context.order_index % data.len();
        let value = data.get(index).unwrap();

        let (_, v) = self.gen_data_with_context("", value, context);
        context.order_index += 1;

        // direct return the value, not the array
        return v;
      } else {
        let mut context = context.clone();
        let count = rule.count.unwrap();
        context.inc = 0;
        for _ in 0..count {
          for v in data {
            let name = result.len().to_string();
            let (_, value) = self.gen_data_with_context(&name, v, &mut context);
            result.push(value);

            context.inc += 1;
          }
        }
      }
    }

    Value::Array(result)
  }

  fn object(
    &self,
    _rule: &Rule,
    data: &Map<String, Value>,
    context: &mut GeneratorContext,
  ) -> Value {
    let mut result = Map::new();
    for (name, value) in data {
      let (n, v) = self.gen_data_with_context(name, value, context);
      result.insert(n, v);
    }

    Value::Object(result)
  }

  /// 'float1|.1-10': 10,
  /// 'float2|1-100.1-10': 1,
  /// 'float3|999.1-10': 1,
  /// 'float4|.3-10': 123.123,
  /// 'grade1|1-100': 1,
  fn number(&self, rule: &Rule, data: &Number, context: &GeneratorContext) -> Value {
    if rule.is_drange {
      let part1 = if rule.is_range {
        rule.count.unwrap()
      } else {
        0
      };

      let dcount = rule.dcount.unwrap();
      let mut part2 = String::new();
      for _ in 0..dcount {
        part2.push(utils::char("number"));
      }

      let num = format!("{}.{}", part1, part2).parse::<f64>().unwrap();
      json!(num)
    } else if rule.is_range {
      json!(rule.count.unwrap())
    } else if rule.is_step {
      let inc = context.inc + rule.step.unwrap();
      json!(inc)
    } else {
      Value::Number(data.clone())
    }
  }

  fn string(&self, rule: &Rule, data: &String, _context: &GeneratorContext) -> Value {
    let mut result = data.clone();

    // 1 function name  2 function parameters
    let re_fn = Regex::new(r"@([^@#%&()\?\s]+)(?:\((.*?)\))?").unwrap();
    let fn_capture = re_fn.captures(data);

    if let Some(cap) = fn_capture {
      let fn_name = cap.get(1).map(|m| m.as_str()).unwrap();

      let mut range: Vec<usize> = Vec::new();
      if let Some(p) = cap.get(2).map(|m| m.as_str()) {
        range = p
          .split(",")
          .map(|item| item.trim().parse::<usize>().unwrap())
          .collect();
      }

      let mut params: (usize, usize) = (3, 10);
      if range.len() == 1 {
        params = (range[0], range[0]);
      } else if range.len() == 2 {
        params = (range[0], range[1]);
      }

      result = call(fn_name, params);
    }

    if let Some(count) = rule.count {
      // it already has one value in result
      // just add count-1 to make sure the result has the item of count
      for _ in 0..count - 1 {
        result.push_str(data);
      }
    }

    Value::String(result)
  }

  fn bool(&self, rule: &Rule, data: &bool, _context: &GeneratorContext) -> Value {
    if rule.is_rule {
      Value::Bool(utils::bool())
    } else {
      Value::Bool(data.clone())
    }
  }
}
