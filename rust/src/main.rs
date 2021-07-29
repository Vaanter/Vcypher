use base64;
use clap::{App, Arg};

#[path = "./core/vcypher.rs"]
mod vcypher;
#[path = "./misc/utils.rs"]
mod utils;

fn main() {
  encode_arg();
}

fn encode_arg() {
  let matches = App::new("Vcypher")
    .version("1.0")
    .author("Valentín Bolfík <wtrol25@gmail.com>")
    .about("Encodes input using the vcypher algorithm")
    .arg(Arg::with_name("base64")
      .short("b")
      .long("base64")
      .takes_value(false)
      .help("Outputs the result in base64 encoding."))
    .arg(Arg::with_name("INPUTS")
      .required(true)
      .multiple(true)
      .index(1)
      .help("Space separated list of inputs to encode."))
    .get_matches();
  let inputs = matches.values_of("INPUTS").unwrap();
  for input in inputs {
    let result = vcypher::vcypher(input);
    if matches.is_present("base64") {
      let base64_result = base64::encode(result);
      println!("{}", base64_result);
    } else {
      println!("{}", result);
    }
  }
}
