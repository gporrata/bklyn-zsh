extern crate getopts;
extern crate bklyn_zsh;

use std::env;
use getopts::Options;
use bklyn_zsh::joiners;

fn help(opts: Options, program: &str) {
  let brief = format!("Usage: {} FILE [options]", program);
  print!("{}", opts.usage(&brief));
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
  else {
    // get prompt 
    matches.opt_str("p").and_then(move |prompt|
      match prompt.as_ref() {
        "zsh-left" => Some(joiners::zshLeft(matches.free)),
        "zsh-right" => Some(joiners::zshRight(matches.free)),
        "tmux-left" => Some(joiners::tmuxLeft(matches.free)),
        "tmux-window" => Some(joiners::tmuxWindowStatus(matches.free)),
        "tmux-right" => Some(joiners::tmuxRight(matches.free)),
        _ => None
      }
    );
  }
}

