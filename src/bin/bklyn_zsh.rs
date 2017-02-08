extern crate getopts;

use std::env;
use getopts::Options;

fn help(opts: Options, program: &str) {
  let brief = format!("Usage: {} FILE [options]", program);
  print!("{}", opts.usage(&brief));
}

fn zsh_left() {
  unimplemented!();    
}

fn zsh_right() {
  unimplemented!();
}

pub fn main() {
  // get arguments and program
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  // eval options
  let mut opts = Options::new();
  opts.optflag("h", "help", "print this help menu");
  opts.optopt("p", "prompt", "prompt to build", "PROMPT");
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };
  
  // get help
  if matches.opt_present("h") {
    help(opts, &program);
  }
  // get prompt 
  match matches.opt_str("p") {
    Some(ref prompt) if prompt == "zsh-left" => zsh_left(),
    Some(ref prompt) if prompt == "zsh-right" => zsh_right(),
    Some(ref prompt) => panic!("Unknown prompt {}", prompt),
    None => {}
  }
}

