use crate::vcypher;
use std::fs;
use std::collections::HashMap;

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
pub fn read_from_file() -> String {
  fs::read_to_string("input.txt").expect("ERROR")
}

#[allow(dead_code)]
pub fn write_to_file(output: &str) {
  let result = fs::write("output.txt", output.to_string());

  match result {
    Ok(_) => (),
    Err(err) => panic!(err)
  };
}
