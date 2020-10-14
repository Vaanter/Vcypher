use base64;
use std::env;

#[path = "./core/vcypher.rs"]
mod vcypher;
#[path = "./misc/utils.rs"]
mod utils;

fn main() {
  // utils::collision();
  // utils::file_as_input_test();
  encode_arg();
}

#[allow(dead_code)]
fn encode_arg() {
  let args: Vec<String> = env::args().collect();
  let input = match args.get(1) {
    Some(input) => input,
    None => panic!("No input specified!")
  };
  let base64_output: bool = match args.get(2) {
    None => false,
    value if value.unwrap_or(&String::new()) == "--base64" => true,
    Some(_) => panic!("Invalid second parameter!")
  };
  let result = vcypher::vcypher(input);
  if base64_output {
    let base64_result = base64::encode(result);
    println!("{}", base64_result);
  } else {
    println!("{}", result);
  }
}
