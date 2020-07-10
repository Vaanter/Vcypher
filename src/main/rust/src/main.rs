mod vcypher;

use base64;

fn main() {
  use std::env;

  let args: Vec<String> = env::args().collect();
  let input = match args.get(1) {
    Some(second) => second,
    None => ""
  };
  let result = vcypher::vcypher(input.to_string());
  println!("{}", result);
  let base64_result = base64::encode(result);
  println!("{}", base64_result);
}
