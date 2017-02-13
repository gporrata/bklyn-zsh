#![allow(non_upper_case_globals)]

extern crate futures;

use ::segments::*;
use ::segments::Segment;
use self::futures::future::*;
use std::env;

const bg0: u32 = 0x094d77;
const fg0: u32 = 0x2ecc71;

fn wrap_icon(icon: &str) -> Vec<Part> {
  vec!(Part::Bg(bg0), Part::Fg(fg0), Part::Text(format!("{} ", icon)))
}

pub fn segment() -> Segment {
  let os = env::var("OSTYPE").expect("Missing OSTYPE!");
  let result = 
    if os == "linux-gnu" {
      // TODO: determine linux distro
      // redhat: "\u{f309}"
      // ubuntu: "\u{fe73a}"
      wrap_icon("\u{f17c}")
    }
    else if os.starts_with("darwin") {
      // TODO: show macos version maybe?
      wrap_icon("\u{f179}")
    }
    else if os == "cygwin" {
      wrap_icon("\u{f17a}") // TODO: eval icon here; generic win icon
    }
    else if os == "msys" {
      wrap_icon("\u{f17a}") // TODO: eval icon here; generic win icon
    }
    else if os == "win32" {
      wrap_icon("\u{f17a}") // TODO: eval icon here; generic win icon
    }
    else if os.starts_with("freebsd") {
      wrap_icon("") // TODO: eval icon here
    }
    else {
      Vec::new() // unknown OS?
    };
  ok(result).boxed()
}
