use num_bigint::BigUint;
use std::collections::VecDeque;

pub fn vcypher(input: &str) -> String {

  let mut binaries = vec![];
  let mut one_counts = vec![];

  for character in input.chars() {
    let binary_value: String = format!("{:08b}", character as u32);
    let one_count = binary_value.matches('1').count();
    one_counts.push(one_count);
    binaries.push(binary_value);
  }

  let mut zero_curve: Vec<usize> = vec![];
  {
    let all_bin = binaries.join("");
    let mut zero_counter = 0;
    for (pos, character) in all_bin.chars().enumerate() {
      if character == '0' {
        zero_counter += 1;
      } else if zero_counter != 0 {
        zero_counter += zero_curve.last().unwrap_or(&0);
        zero_curve.push(zero_counter);
        zero_counter = 0;
      }
      if (pos == all_bin.len() - 1) && zero_counter != 0 {
        zero_counter += zero_curve.last().unwrap_or(&0);
        zero_curve.push(zero_counter);
      }
    }
  }

  let mut pre_rot = Vec::with_capacity(zero_curve.len() + 1);

    for character in zero_curve {
      pre_rot.push(character.to_string());
    }

  {
    let one_counts_product: BigUint = one_counts.iter().product();
    pre_rot.push(one_counts_product.to_string());
  }

  let mut rot_values: VecDeque<String> = VecDeque::with_capacity(pre_rot.len());
  for character in pre_rot.iter() {
    let reversed = character.chars().rev().collect();
    rot_values.push_back(reversed);
  }

  let mut encrypted = rot_values.pop_back().unwrap_or(String::new()).to_string();

  for i in 1..rot_values.len() {
    for _ in 0..2 {
      if i % 2 == 0 {
        encrypted += rot_values.pop_back().unwrap_or(String::new()).as_str();
      } else {
        encrypted += rot_values.pop_front().unwrap_or(String::new()).as_str();
      }
    }
  }

  let mut zeros_to_remove = 0;
  for num in encrypted.chars() {
    if num != '0' {
      break;
    }
    zeros_to_remove += 1;
  }
  encrypted[zeros_to_remove..].to_string()
}
