use num_bigint::BigUint;
use std::collections::VecDeque;

pub fn vcypher(payload: &str) -> String {
  let mut binaries = vec![];
  let mut one_counts = vec![];

  for character in payload.chars() {
    let binary_value: String = format!("{:08b}", character as u32);
    let one_count = binary_value.matches('1').count();
    one_counts.push(one_count);
    binaries.push(binary_value);
  }

  let mut zero_curve: Vec<usize> = vec![];
  {
    let all_bin = binaries.join("");
    let zero_split: Vec<&str> = all_bin.split("1").filter(|x| x != &"").collect();

    for zeros in zero_split {
      let length = zeros.len() + zero_curve.last().unwrap_or(&0);
      zero_curve.push(length);
    }
  }

  let mut pre_rot: Vec<String> = zero_curve.iter().map(|x| x.to_string()).collect();
  if one_counts.len() > 0 {
    let one_counts_product: BigUint = one_counts.iter().product();
    
    pre_rot.push(one_counts_product.to_string());
  }

  let mut rot_values: VecDeque<String> =
    pre_rot.iter().map(|x| x.chars().rev().collect()).collect();

  let mut encrypted = rot_values.pop_back().unwrap_or(String::new()).to_string();

  for i in 0..rot_values.len() / 2 + 1 {
    for _ in 0..2 {
      if i % 2 != 0 {
        encrypted += rot_values.pop_back().unwrap_or(String::new()).as_str();
      } else {
        encrypted += rot_values.pop_front().unwrap_or(String::new()).as_str();
      }
    }
  }
  if encrypted == "" {encrypted} else {encrypted.parse::<BigUint>().unwrap().to_string()}
}
