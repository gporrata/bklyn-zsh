#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use ::segments::*;
use std::env;

const bg0: u32 = 0xd35400;
const fg0: u32 = 0xf1C40f;
const fg1: u32 = 0xffffff;

pub fn segment() -> Option<Vec<Part>> {
  if env::var("SSH_TTY").ok().map_or(0, |tty| tty.len()) > 0 {
    let user = env::var("USER").ok().unwrap_or_else(|| "".to_string());
    let host = env::var("HOST").ok().unwrap_or_else(|| "".to_string());
    let result = vec![
      Part::Bg(bg0), Part::Fg(fg0), Part::StaticText("\u{f0c2}  "), 
      Part::Fg(fg1), Part::Text(user), Part::StaticText("@"), Part::Text(host)
    ];
    Some(result)
  }
  else {
    None
  }
}


