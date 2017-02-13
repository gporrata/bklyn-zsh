#![allow(non_snake_case)]

extern crate futures;

mod osSegment;
mod dirSegment;
mod gitSegment;

use std::vec::Vec;

use self::futures::future::{BoxFuture};

type Segment = BoxFuture<Vec<Part>, ()>;

// part of segment that will be rendered
pub enum Part {
  Text(String),
  Fg(u32), // color code
  Bg(u32), // color code
  FgReset {},
  BgReset {},
  Ignore {}
}

pub fn segment_of(name: &str) -> Option<Segment> {
  match name {
    "os" => Some(osSegment::segment()),
    "dir" => Some(dirSegment::segment()),
    "git" => Some(gitSegment::segment()),
    _ => None
  }
}

