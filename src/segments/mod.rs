#![allow(non_snake_case)]

extern crate futures;

mod osSegment;
mod dirSegment;

use std::vec::Vec;

use self::futures::future::{BoxFuture};

pub fn segment_of(name: &str) -> Option<Box<Segment>> {
  match name {
    "os" => Some(Box::new(osSegment::OsSegment::new())),
    "dir" => Some(Box::new(dirSegment::DirSegment::new())),
    _ => None
  }
}

// part of segment that will be rendered
pub enum Part {
  Text(String),
  Fg(u32), // color code
  Bg(u32), // color code
  FgReset {},
  BgReset {}
}

// get segment data
pub trait Segment {
  // eval text to render, resolved or not, we must have something
  fn text(&self) -> BoxFuture<Vec<Part>, ()>;
}
