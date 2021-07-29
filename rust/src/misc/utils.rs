use crate::vcypher;
use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;

#[allow(dead_code)]
pub fn collision() {
  let mut occurences: HashMap<String, Vec<String>> = HashMap::new();
  for input in 65..91 {
    let input_string = (input as u8 as char).to_string();
    let result = vcypher::vcypher(&input_string);
    if occurences.contains_key(&result) {
      let inputs = occurences.get_mut(&result).unwrap();
      (*inputs).push(input_string);
    } else {
      occurences.insert(result, vec![input_string]);
    }
  }
  println!("{:#?}", occurences);
}

#[allow(dead_code)]
pub fn file_as_input_test() {
  let input = read_from_file();
  let start = SystemTime::now();
  let result = vcypher::vcypher(&input);
  println!("{}ns", start.elapsed().unwrap().as_nanos());
  write_to_file(&result);
}

#[allow(dead_code)]
pub fn read_from_file() -> String {
  fs::read_to_string("input.txt").expect("ERROR")
}

#[allow(dead_code)]
pub fn write_to_file(output: &str) {
  let result = fs::write("output.txt", output.to_string());

  if let Err(err) = result {
    panic!("Error writing to file, {}", err);
  }
}
