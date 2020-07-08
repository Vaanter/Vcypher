extern crate num_bigint;

use num_bigint::BigUint;
use std::collections::VecDeque;

pub fn vcypher(input: String) -> String {

  let mut binaries: Vec<String> = vec![];
  let mut one_counts: Vec<usize> = vec![];

  for character in input.chars() {
    let binary_value: String = format!("{:08b}", character as u32);
    let one_count: usize = binary_value.matches('1').count();
    one_counts.push(one_count);
    binaries.push(binary_value)
  }

  let mut zero_curve: Vec<usize> = vec![];
  {
    let all_bin = binaries.join("");
    let mut zero_counter = 0;
    for (pos, character) in all_bin.chars().enumerate() {
      if character == '0' {
        zero_counter += 1
      } else if zero_counter != 0 {
        if !zero_curve.is_empty() {
          zero_counter += *zero_curve.last().unwrap();
        }
        zero_curve.push(zero_counter);
        zero_counter = 0;
      }
      if (pos == all_bin.len() - 1) && zero_counter != 0 {
        if !zero_curve.is_empty() {
          zero_counter += *zero_curve.last().unwrap();
        }
        zero_curve.push(zero_counter);
      }
    }
  }

  let mut pre_rot: Vec<String> = vec![];

  {
    for character in zero_curve.iter() {
      pre_rot.push(ToString::to_string(&character));
    }

    let one_counts_product: BigUint = one_counts.iter().product();
    pre_rot.push(ToString::to_string(&one_counts_product));
  }

  let mut rot_values: VecDeque<String> = VecDeque::new();
  for character in pre_rot.iter() {
    if character.len() >= 2 {
      let reversed: String = character.chars().rev().collect();
      rot_values.push_back(reversed);
    } else {
      rot_values.push_back(character.to_string());
    }
  }

  let mut encrypted: String = String::new();

  for i in 0..rot_values.len() {
    if i == 0 {
      encrypted += rot_values.pop_back().unwrap().as_str();
    } else if rot_values.len() >= 2 {
      for _ in 0..2 {
        if i % 2 == 0 {
          encrypted += rot_values.pop_back().unwrap().as_str();
        } else {
          encrypted += rot_values.pop_front().unwrap().as_str();
        }
      }
    } else if !rot_values.is_empty() {
      encrypted += rot_values.pop_back().unwrap().as_str();
    }
  }

  let mut ok = String::new();
  let mut zeros_to_remove = 0;
  for num in encrypted.chars() {
    if num == '0' {
      zeros_to_remove += 1;
    } else {
      ok = (&encrypted[zeros_to_remove..]).parse().unwrap();
      break;
    }
  }
  ok
}
