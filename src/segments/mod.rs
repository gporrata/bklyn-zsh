#![allow(non_snake_case)]

extern crate futures;

mod osSegment;

use std::collections::HashMap;
use std::vec::Vec;
use std::env;

use self::futures::future::{BoxFuture};

// get environment variables as hashmap
// non-unicode strings are dropped 
pub fn environment() -> HashMap<String, String> {
  env::vars_os()
    .filter_map(|e| {
      let k = e.0.into_string();
      let v = e.1.into_string();
      match (k, v) {
        (Ok(kstr), Ok(vstr)) => Some((kstr, vstr)),
        _ => None
      }
    })
    .collect() 
}

pub fn segment_of(name: &str) -> Option<Box<Segment>> {
  match name {
    "os" => Some(Box::new(osSegment::OsSegment::new())),
    _ => None
  }
}

// hold environment variables and other segment specific functions
pub struct Environment {
  pub env: HashMap <String, String>
}

// part of segment that will be rendered
pub enum Part {
  Text(String),
  Fg(u32), // color code
  Bg(u32), // color code
  FgClear {},
  BgClear {}
}

// get segment data
pub trait Segment {
  // return background color to use
  fn bg(&self) -> u32;
  // eval text to render, resolved or not, we must have something
  fn text(&self, env: &Environment) -> BoxFuture<Vec<Part>, ()>;
}
