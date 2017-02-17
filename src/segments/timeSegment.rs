#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use ::segments::*;
use std::process::Command;

const bg0: u32 = 0x2C3E50;
const fg0: u32 = 0x27ae60;
const fg1: u32 = 0xcccccc;

const timeicon: &'static str = "\u{f017} ";
const sep: &'static str = " \u{f142}";

fn time() -> Option<String> {
  Command::new("date")
  .arg("+%X")
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
}

pub fn segment() -> Option<Vec<Part>> {
  time().map(|time|{
    vec![
      Part::Bg(bg0),
      Part::Fg(fg0),
      Part::StaticText(timeicon),
      Part::Fg(fg1),
      Part::Text(time)
    ]
  })
}
