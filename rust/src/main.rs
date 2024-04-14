use base64::Engine;
use clap::Parser;

#[path = "./misc/utils.rs"]
mod utils;
#[path = "./core/vcypher.rs"]
mod vcypher;

#[derive(Parser)]
#[command(
  author = "Valentín Bolfík <wtrol25@gmail.com>",
  version,
  about = "Encodes input using the vcypher algorithm"
)]
struct Args {
  #[arg(short, long, default_value = "false")]
  base64: bool,
  inputs: Vec<String>,
}

fn main() {
  encode_arg();
}

fn encode_arg() {
  let args = Args::parse();
  for input in &args.inputs {
    let result = vcypher::vcypher(input);
    if args.base64 {
      let base64_result = base64::prelude::BASE64_STANDARD_NO_PAD.encode(result);
      println!("{}", base64_result);
    } else {
      println!("{}", result);
    }
  }
}
