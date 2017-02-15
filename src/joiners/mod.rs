#![allow(non_snake_case)]
#![allow(unused_variables)]

mod zsh;
mod tmux;

use std::vec::Vec;
use segments::*;

trait ColorOperations {
  fn all_reset(&self) -> &'static str;
  fn fg_reset(&self) -> &'static str;
  fn bg_reset(&self) -> &'static str;
  fn fg(&self, color: u32) -> String;
  fn bg(&self, color: u32) -> String;
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
fn left_sep<F1>(
  result: &mut String, cop: &F1,
  seps: SepCodes, hasLast: bool, lastBg: u32, currBg: u32) 
where F1: ColorOperations {
  if hasLast {
    if currBg == lastBg {
      //result.push_str(seps[1][1]);
    }
    else {
      result.push_str(&cop.fg(lastBg));
      if currBg == 0 {
        result.push_str(cop.bg_reset());
      }
      else {
        result.push_str(&cop.bg(currBg));
      }
      result.push_str(seps[1][0]);
      result.push_str(" ");
    }
  }
}

// adds right separator
fn right_sep<F1>(
  result: &mut String, cop: &F1,
  seps: SepCodes, hasLast: bool, lastBg: u32, currBg: u32) 
where F1: ColorOperations {
  if hasLast {
    result.push_str(&cop.bg(lastBg));
  }
  result.push_str(&cop.fg(currBg));
  result.push_str(seps[1][2]);
}

// combine texts for left prompt
fn parts_fold<F1, F2, F3>(
  texts: Vec<Vec<Part>>, cop: &F1, left: bool, prefixSegment: F2, postfix: F3) -> String 
where
  F1: ColorOperations,
  F2: Fn(&mut String, bool, u32, u32),
  F3: FnOnce(&mut String, bool, u32)
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
        Part::Fg(color) => result.push_str(&cop.fg(color)), 
        Part::Bg(color) => result.push_str(&cop.bg(color)),
        Part::FgReset{} => result.push_str(cop.fg_reset()), 
        Part::BgReset{} => result.push_str(cop.bg_reset()),
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
pub fn zshLeft(segments: Vec<String>) { 
  let seps = sep_codes();
  let cop = zsh::cop();
  let texts = segments.iter()
    .flat_map(|segment| segment_of(segment))
    .collect();
  let string = parts_fold(texts, &cop, true,
    |mut stringBuf, hasLast, lastBg, currBg| 
      left_sep(&mut stringBuf, &cop, seps, hasLast, lastBg, currBg),
    |mut stringBuf, hasLast, lastBg| {
      left_sep(&mut stringBuf, &cop, seps, true, lastBg, 0);
      stringBuf.push_str(cop.all_reset());
      stringBuf.push_str("\n\u{f489}  ");
    });
  println!("{}", string.as_str());
}

// generate zsh right prompt
pub fn zshRight(segments: Vec<String>) {
  let seps = sep_codes();
  let cop = zsh::cop();
  let texts = segments.iter()
    .flat_map(|segment| segment_of(segment))
    .collect();
  let string = parts_fold(texts, &cop, false,
    |mut stringBuf, hasLast, lastBg, currBg|
      right_sep(&mut stringBuf, &cop, seps, hasLast, lastBg, currBg),
    |mut stringBuf, hasLast, lastBg| {
      stringBuf.push_str(cop.all_reset());
    });
  print!("{}", string.as_str());
}

// generate tmux left prompt
pub fn tmuxLeft(segments: Vec<String>) {
  let seps = sep_codes();
  let cop = tmux::cop();
  let texts = segments.iter()
    .flat_map(|segment| segment_of(segment))
    .collect();
  let string = parts_fold(texts, &cop, true,
    |mut stringBuf, hasLast, lastBg, currBg| 
      left_sep(&mut stringBuf, &cop, seps, hasLast, lastBg, currBg),
    |mut stringBuf, hasLast, lastBg| {
      left_sep(&mut stringBuf, &cop, seps, true, lastBg, 0);
      stringBuf.push_str(cop.all_reset());
    });
  println!("{}", string.as_str());
}
