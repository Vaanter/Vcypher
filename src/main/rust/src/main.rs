use base64;
use std::env;

mod vcypher;
mod test;

fn main() {
  // test::collision();
  // test::write_to_file(&vcypher::vcypher(&test::read_from_file()));
  encode_arg();
}

#[allow(dead_code)]
fn encode_arg() {
  let args: Vec<String> = env::args().collect();
  let input = match args.get(1) {
    Some(input) => input,
    None => panic!("No input specified!")
  };
  let result = vcypher::vcypher(input);
  println!("{}", result);
  let base64_result = base64::encode(result);
  println!("{}", base64_result);
}
