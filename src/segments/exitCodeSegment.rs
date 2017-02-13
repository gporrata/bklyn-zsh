#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use ::segments::*;
use std::env;

const fgSuccess: u32 = 0x033d1a;
const fgFailed: u32 = 0xC0392B;
const bg0: u32 = 0xECF0F1;
const fg0: u32 = 0x000000;

pub fn segment() -> Option<Vec<Part>> {
  env::var("bklyn_zsh_exit_code")
    .ok()
    .and_then(|exitCode| exitCode.parse::<i32>().ok())
    .map(|exitCode|{
      if exitCode == 0 {
        vec![
          Part::Bg(bg0),
          Part::Fg(fgSuccess),
          Part::Text("\u{f00c}  ".to_string()),
        ]
      }
      else {
        vec![
          Part::Bg(bg0),
          Part::Fg(fgFailed),
          Part::Text("\u{f00d} ".to_string()),
          Part::Fg(fg0),
          Part::Text(exitCode.to_string())
        ]
      }
    })
}
