#![allow(non_upper_case_globals)]

extern crate regex;

use ::segments::*;
use std::process::Command;
use self::regex::*;

// use for time
//const bg0: u32 = 0x2C3E50;
const bg0: u32 = 0xecf0f1;
const fg0: u32 = 0x27ae60;
const fg1: u32 = 0x0f3247;

const avgicon: &'static str = "\u{f012}  ";
const sep: &'static str = " \u{f142}";

struct LoadAvg {
  la_1min: String,
  la_5min: String,
  la_15min: String
}

fn loadAvg() -> Option<LoadAvg> {
  let larex = Regex::new(r"load average: (\d+(\.\d+)), (\d+(\.\d+)), (\d+(\.\d+))").unwrap();
  Command::new("uptime")
  .output()
  .ok()
  .and_then(|output|
    if output.status.success() {
      String::from_utf8(output.stdout).ok()
    }
    else {
      None
    }
  )
  .and_then(|out| {
    larex.captures(&out)
    .map(|captures| {
      let la_1min = captures.get(1).unwrap().as_str();
      let la_5min = captures.get(3).unwrap().as_str();
      let la_15min = captures.get(5).unwrap().as_str();
      LoadAvg { 
        la_1min: String::from(la_1min), 
        la_5min: String::from(la_5min), 
        la_15min: String::from(la_15min)
      }
    })
  })
}

pub fn segment() -> Option<Vec<Part>> {
  loadAvg().map(|load|{
    vec![
      Part::Bg(bg0),
      Part::Fg(fg0),
      Part::StaticText(avgicon),
      Part::Fg(fg1),
      Part::Text(load.la_1min),
      Part::Fg(fg0),
      Part::StaticText(sep),
      Part::Fg(fg1),
      Part::Text(load.la_5min),
      Part::Fg(fg0),
      Part::StaticText(sep),
      Part::Fg(fg1),
      Part::Text(load.la_15min)
    ]
  })
}
