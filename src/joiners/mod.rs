#![allow(non_snake_case)]
#![allow(unused_variables)]

mod zsh;
mod tmux;

extern crate regex;

use std::vec::Vec;
use segments::*;
use self::regex::Regex;
use std::env;

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

// adds separator on the right side of segments; for zsh left prompt for instance
fn right_sep<F1>(
  result: &mut String, cop: &F1,
  seps: SepCodes, lastBg: u32, currBg: u32) 
where F1: ColorOperations {
  if currBg == lastBg {
    //result.push_str(seps[1][1]);
  }
  else {
    if lastBg == 0 {
      result.push_str(&cop.fg_reset());
    }
    else {
      result.push_str(&cop.fg(lastBg));
    }
    if currBg == 0 {
      result.push_str(cop.bg_reset());
    }
    else {
      result.push_str(&cop.bg(currBg));
    }
    result.push_str(seps[1][0]);
  }
}

// adds separator on the right side of segments; for zsh left prompt for instance
fn right_sep2<F1>(
  result: &mut String, cop: &F1,
  seps: SepCodes, lastBg: u32, currBg: u32) 
where F1: ColorOperations {
  if currBg == lastBg {
    //result.push_str(seps[1][1]);
  }
  else {
    if lastBg == 0 {
      result.push_str(&cop.fg_reset());
    }
    else {
      result.push_str(&cop.fg(lastBg));
    }
    if currBg == 0 {
      result.push_str(cop.bg_reset());
    }
    else {
      result.push_str(&cop.bg(currBg));
    }
    result.push_str(seps[1][1]);
  }
}

// adds separator to the left side of a segment (for zsh right prompt for instance)
fn left_sep<F1>(
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
      if hasLast {
        right_sep(&mut stringBuf, &cop, seps, lastBg, currBg);
        stringBuf.push_str(" ");
      },
    |mut stringBuf, hasLast, lastBg| {
      right_sep(&mut stringBuf, &cop, seps, lastBg, 0);
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
      left_sep(&mut stringBuf, &cop, seps, hasLast, lastBg, currBg),
    |mut stringBuf, hasLast, lastBg| {
      stringBuf.push_str(cop.all_reset());
    });
  print!("{}", string.as_str());
}

// generate tmux status left
pub fn tmuxLeft(segments: Vec<String>) {
  let seps = sep_codes();
  let cop = tmux::cop();
  let texts = segments.iter()
    .flat_map(|segment| segment_of(segment))
    .collect();
  let string = parts_fold(texts, &cop, true,
    |mut stringBuf, hasLast, lastBg, currBg| 
      if hasLast {
        right_sep(&mut stringBuf, &cop, seps, lastBg, currBg)
      },
    |mut stringBuf, hasLast, lastBg| {
      right_sep(&mut stringBuf, &cop, seps, lastBg, 0);
      stringBuf.push_str(cop.all_reset());
      stringBuf.push_str(" ");
    });
  println!("{}", string.as_str());
}

pub struct WindowStatus {
  pub windowIndex: u32,
  pub numWindows: u32,
  pub isActive: bool
}

pub fn getTmuxWindowStatusEnv() -> WindowStatus {
  let tmuxStatus = env::var("tmux_window_status").expect("Missing tmux_window_status!");
  let rex = Regex::new(r"(\d+):(\d+):(1|0)").unwrap();
  rex.captures(&tmuxStatus).map(|captures| {
    let windowIndex = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let numWindows = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let isActive = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
    WindowStatus { windowIndex: windowIndex, numWindows: numWindows, isActive: isActive != 0 }
  })
  .unwrap()
}

// generate tmux window status
pub fn tmuxWindowStatus(segments: Vec<String>) {
  let seps = sep_codes();
  let cop = tmux::cop();
  let windowStatus = getTmuxWindowStatusEnv();
  let texts = segments.iter()
    .flat_map(|segment| segment_of(segment))
    .collect();
  let string = parts_fold(texts, &cop, true,
    |mut stringBuf, hasLast, lastBg, currBg|
      if hasLast {
        right_sep(&mut stringBuf, &cop, seps, lastBg, currBg)
      }
      else if windowStatus.windowIndex == 0 {
        left_sep(&mut stringBuf, &cop, seps, false, lastBg, currBg)
      }
      else if windowStatus.isActive {
        right_sep(&mut stringBuf, &cop, seps, 
          windowStatusSegment::bg0_inactive, windowStatusSegment::bg0_active)
      }
      else {
        right_sep2(&mut stringBuf, &cop, seps, 
          windowStatusSegment::bg0_active, windowStatusSegment::bg0_inactive)
      },
    |mut stringBuf, hasLast, lastBg| {
      if windowStatus.windowIndex < (windowStatus.numWindows - 1) {
        right_sep(&mut stringBuf, &cop, seps, lastBg, windowStatusSegment::bg0_inactive);
      }
      else {
        right_sep(&mut stringBuf, &cop, seps, lastBg, 0);
      }
      stringBuf.push_str(cop.all_reset());
    });
  println!("{}", string.as_str());
}

