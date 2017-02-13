#![allow(non_snake_case)]

mod osSegment;
mod dirSegment;
mod gitSegment;
mod exitCodeSegment;
mod pidSegment;

use std::vec::Vec;

// part of segment that will be rendered
pub enum Part {
  Text(String),
  Fg(u32), // color code
  Bg(u32), // color code
  FgReset {},
  BgReset {},
  Ignore {}
}

pub fn segment_of(name: &str) -> Option<Vec<Part>> {
  match name {
    "os" => osSegment::segment(),
    "dir" => dirSegment::segment(),
    "git" => gitSegment::segment(),
    "exit" => exitCodeSegment::segment(),
    "pid" => pidSegment::segment(),
    _ => None
  }
}

