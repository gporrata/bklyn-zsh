#![allow(non_upper_case_globals)]

extern crate futures;

use std::vec::Vec;
use segments::*;
use self::futures::future::*;

const fg_clear: &'static str = "\u{1b}[38m";
const bg_clear: &'static str = "\u{1b}[48m";

// set foreground color
// TODO: cache values
fn fg(color: u32) -> String {
  let b = color & 0xff;
  let g = (color >> 8) & 0xff;
  let r = (color >> 16) & 0xff;
  format!("\u{1b}[38;2;{};{};{}m", r, g, b) 
}

// set background color
// TODO: cache values
fn bg(color: u32) -> String {
  let b = color & 0xff;
  let g = (color >> 8) & 0xff;
  let r = (color >> 16) & 0xff;
  format!("\u{1b}[48;2;{};{};{}m", r, g, b) 
}

// combine texts for left prompt
fn left_fold(texts: Vec<Vec<Part>>) -> String {
  let mut result = String::with_capacity(1000);
  for text in texts {
    for part in text {
      match part {
        Part::Text(string) => result.push_str(&string),
        Part::Fg(color) => result.push_str(&fg(color)), 
        Part::Bg(color) => result.push_str(&bg(color)), 
        Part::FgClear{} => result.push_str(fg_clear), 
        Part::BgClear{} => result.push_str(bg_clear) 
      };
    }
  };
  result
}

// generate zsh left prompt
pub fn left(segments: Vec<String>) { 
  let env = Environment {
    env: environment()
  };
  let futs: Vec<BoxFuture<Vec<Part>, ()>> = segments.iter()
    .map(|string| string.as_str())
    .flat_map(|str| segment_of(str))
    .map(|segment| segment.text(&env))
    .collect();
  let fut = join_all(futs)
    .then(|result| {
      match result {
        Ok(texts) => {
          let string = left_fold(texts);
          print!("{}", string.as_str());
          Ok(string)
        },
        Err(e) => Err(e)
      }
    });
  let _ = fut.wait();
  ()
}

// generate zsh right prompt
pub fn right(segments: Vec<String>) {
  unimplemented!();
}


