#![allow(non_upper_case_globals)]

use ::segments::*;
use std::env;

const bg0: u32 = 0x094d77;
const fg0: u32 = 0x2ecc71;

fn wrap_icon(icon: &'static str) -> Option<Vec<Part>> {
  Some(vec![Part::Bg(bg0), Part::Fg(fg0), Part::StaticText(icon)])
}

pub fn segment() -> Option<Vec<Part>> {
  let os = env::var("OSTYPE").expect("Missing OSTYPE!");
  if os == "linux-gnu" {
    // TODO: determine linux distro
    // redhat: "\u{f309}"
    // ubuntu: "\u{fe73a}"
    wrap_icon(" \u{f17c} ")
  }
  else if os.starts_with("darwin") {
    // TODO: show macos version maybe?
    wrap_icon(" \u{f179} ")
  }
  else if os == "cygwin" {
    wrap_icon(" \u{f17a} ") // TODO: eval icon here; generic win icon
  }
  else if os == "msys" {
    wrap_icon(" \u{f17a} ") // TODO: eval icon here; generic win icon
  }
  else if os == "win32" {
    wrap_icon(" \u{f17a} ") // TODO: eval icon here; generic win icon
  }
  else if os.starts_with("freebsd") {
    wrap_icon("") // TODO: eval icon here
  }
  else {
    None // unknown OS?
  }
}

