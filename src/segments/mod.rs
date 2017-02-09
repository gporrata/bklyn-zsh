#![allow(non_snake_case)]
mod osSegment;

//extern crate futures;

//use futures::future::{Future,Map};
use std::collections::HashMap;
use std::vec::Vec;
use std::env;

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

// hold environment variables and other segment specific functions
struct Environment {
  env: HashMap <String, String>
}

// part of segment that will be rendered
enum Part {
  Text(String)
}

// get segment data
trait Segment {
  fn get_text(&self, env: &Environment) -> &Vec<Part>;
}
