#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::vec::Vec;
use segments::*;

const all_reset: &'static str = "%{\u{1b}[0m%}";
const fg_reset: &'static str = "%{\u{1b}[39m%}"; // or is it 38?
const bg_reset: &'static str = "%{\u{1b}[49m%}"; //  or is it 48?

// set foreground color
// TODO: cache values
fn fg(color: u32) -> String {
  let b = color & 0xff;
  let g = (color >> 8) & 0xff;
  let r = (color >> 16) & 0xff;
  format!("{}\u{1b}[38;2;{};{};{}m{}", "%{", r, g, b, "%}") 
}

// set background color
// TODO: cache values
fn bg(color: u32) -> String {
  let b = color & 0xff;
  let g = (color >> 8) & 0xff;
  let r = (color >> 16) & 0xff;
  format!("{}\u{1b}[48;2;{};{};{}m{}", "%{", r, g, b, "%}") 
}

type SepCodes = [[&'static str; 4]; 4];

// seps
fn sep_codes() -> SepCodes {
  [
    // angles
    ["\u{e0b0}", "\u{e0b1}", "\u{e0b2}", "\u{e0b3}"],
    // curves
    ["\u{e0b4}", "\u{e0b5}", "\u{e0b6}", "\u{e0b7}"],
    // flames
    ["\u{e0c0}", "\u{e0c1}", "\u{e0c2}", "\u{e0c3}"],
    // digital
    ["\u{e0c4}", "\u{e0c5}", "\u{e0c6}", "\u{e0c7}"]
  ]
}

// adds left separator
fn left_sep(result: &mut String, seps: SepCodes, hasLast: bool, lastBg: u32, currBg: u32) {
  if hasLast {
    if currBg == lastBg {
      //result.push_str(seps[1][1]);
    }
    else {
      result.push_str(&fg(lastBg));
      result.push_str(&bg(currBg));
      result.push_str(seps[1][0]);
      result.push_str(" ");
    }
  }
}

// adds right separator
fn right_sep(result: &mut String, seps: SepCodes, hasLast: bool, lastBg: u32, currBg: u32) {
  if hasLast {
    result.push_str(&bg(lastBg));
  }
  result.push_str(&fg(currBg));
  result.push_str(seps[1][2]);
}

// combine texts for left prompt
fn parts_fold<F1, F2>(
  texts: Vec<Vec<Part>>, left: bool, prefixSegment: F1, postfix: F2) -> String 
where
  F1: Fn(&mut String, bool, u32, u32),
  F2: FnOnce(&mut String, bool, u32)
{
  let mut result = String::with_capacity(1000);
  let mut hasLast = false;
  let mut lastBg = 0;
  for text in texts {
    let currBg = match text.first() {
      Some(&Part::Bg(bg)) => bg,
      _ => panic!["Segment without bg??"]
    };
    prefixSegment(&mut result, hasLast, lastBg, currBg);
    let mut firstText = true;
    for part in text {
      match part {
        Part::Text(string) => {
          if left && !hasLast && firstText {
            result.push_str(" ");
            firstText = false;
          }
          result.push_str(&string);
        },
        Part::StaticText(string) => {
          if left && !hasLast && firstText {
            result.push_str(" ");
            firstText = false;
          }
          result.push_str(string);
        },
        Part::Fg(color) => result.push_str(&fg(color)), 
        Part::Bg(color) => result.push_str(&bg(color)),
        Part::FgReset{} => result.push_str(fg_reset), 
        Part::BgReset{} => result.push_str(bg_reset),
        Part::Ignore{} => {}
      };
    }
    hasLast = true;
    lastBg = currBg;
  };
  postfix(&mut result, hasLast, lastBg);
  result
}

// generate zsh left prompt
pub fn left(segments: Vec<String>) { 
  let seps = sep_codes();
  let texts = segments.iter()
    .flat_map(|segment| segment_of(segment))
    .collect();
  let string = parts_fold(texts, true,
    |mut stringBuf, hasLast, lastBg, currBg| 
      left_sep(&mut stringBuf, seps, hasLast, lastBg, currBg),
    |mut stringBuf, hasLast, lastBg| {
      left_sep(&mut stringBuf, seps, true, lastBg, 0);
      stringBuf.push_str(all_reset);
      stringBuf.push_str("\n\u{f489}  ");
    });
  println!("{}", string.as_str());
}

// generate zsh right prompt
pub fn right(segments: Vec<String>) {
  let seps = sep_codes();
  let texts = segments.iter()
    .flat_map(|segment| segment_of(segment))
    .collect();
  let string = parts_fold(texts, false,
    |mut stringBuf, hasLast, lastBg, currBg|
      right_sep(&mut stringBuf, seps, hasLast, lastBg, currBg),
    |mut stringBuf, hasLast, lastBg| {
      stringBuf.push_str(all_reset);
    });
  print!("{}", string.as_str());
}


