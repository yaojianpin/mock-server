// use crate::template::get_fn_mapping;
// use crate::template::Value;
// use regex::Regex;
// use serde_json::json;

// pub fn parse_data_value(value: &Value, count: u32) -> Vec<Value> {
//   let mut tpl = value.clone();
//   if value.is_array() {
//     tpl = value.as_array().unwrap()[0].clone();
//   }

//   let obj = tpl.as_object().unwrap();
//   let serial_regex = Regex::new(RE_SERIAL_NUM).unwrap();
//   let fn_regex = Regex::new(RE_FN_NAME).unwrap();
//   let mut list: Vec<Value> = Vec::new();
//   for index in 0..count {
//     let mut o = obj.clone();
//     for (_, v) in o.iter_mut() {
//       tracing::debug!("parse_data_value, v={:?}", v);
//       let sn_capture = serial_regex.captures(v.as_str().unwrap());

//       if let Some(cap) = sn_capture {
//         let num: u32 = cap
//           .get(1)
//           .map(|m| m.as_str())
//           .unwrap()
//           .parse::<u32>()
//           .unwrap_or(0);

//         *v = json!(num + index);
//         continue;
//       }

//       let fn_capture = fn_regex.captures(v.as_str().unwrap());
//       if let Some(cap) = fn_capture {
//         let fn_name = cap.get(1).map(|m| m.as_str()).unwrap();
//         let mut fn_params = (3, 5);
//         if let Some(fn_params_str) = cap.get(2).map(|m| m.as_str()) {
//           let range: Vec<usize> = fn_params_str
//             .split(",")
//             .map(|item| item.trim().parse::<usize>().unwrap())
//             .collect();

//           if range.len() == 1 {
//             fn_params = (range[0], range[0])
//           } else {
//             fn_params = (range[0], range[1])
//           }
//         }

//         //tracing::debug!("parse_data_value, fn={:?}, params={:?}", fn_name, fn_params);
//         if let Some(func) = get_fn_mapping().get(fn_name) {
//           *v = json!(func(fn_params.0, fn_params.1));
//         }

//         continue;
//       }
//     }

//     list.push(Value::Object(o))

//     // re_set.iter().for_each(|re| {
//     //   let captures = re.captures(v.as_str().unwrap());
//     //   if captures.is_some() {
//     //     tracing::debug!("parse_data_value, {:?}", captures)
//     //   }
//     // });

//     //if re.captures(v.as_str().unwrap()) {}
//   }

//   tracing::debug!("parse_data_value, obj={:?}", obj);

//   list
// }
