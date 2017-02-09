#![allow(non_upper_case_globals)]

extern crate futures;

use std::collections::HashMap;
use ::segments::{Segment,Environment,Part};
use self::futures::future::{BoxFuture,ok};

pub struct OsSegment {}

const bg0: u32 = 0x094d77;
const fg0: u32 = 0x2ecc71;

fn wrap_icon(icon: &str) -> Vec<Part> {
  vec!(Part::Bg(bg0), Part::Fg(fg0), Part::Text(format!("{} ", icon)))
}

impl OsSegment {
  pub fn new() -> OsSegment {
    OsSegment {}
  }
}

impl Segment for OsSegment {
  fn bg(&self) -> u32 {
    bg0
  }
  fn text(&self, env: &Environment) -> BoxFuture<Vec<Part>, ()> {
    let result: Vec<Part> = env.env.get("OSTYPE")
      .map_or_else(
        || {
          Vec::new() // unknown OS?
        },
        |os| {       
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
          }
        }
      );
    Box::new(ok(result))
  }
}
