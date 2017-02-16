#![allow(non_snake_case)]

mod osSegment;
mod sshSegment;
mod dirSegment;
mod gitSegment;
mod exitCodeSegment;
mod pidSegment;
pub mod windowStatusSegment;
mod loadSegment;
mod customSegment; // TODO: remove

use std::vec::Vec;

// part of segment that will be rendered
pub enum Part {
  Text(String),
  StaticText(&'static str),
  Fg(u32), // color code
  Bg(u32), // color code
  FgReset {},
  BgReset {},
  Ignore {}
}

pub fn segment_of(name: &str) -> Option<Vec<Part>> {
  match name {
    "os" => osSegment::segment(),
    "ssh" => sshSegment::segment(),
    "dir" => dirSegment::segment(),
    "git" => gitSegment::segment(),
    "exit" => exitCodeSegment::segment(),
    "pid" => pidSegment::segment(),
    "load" => loadSegment::segment(),
    str if str.starts_with("window:") => windowStatusSegment::segment(name),
    _ => customSegment::segment(name) 
  }
}

