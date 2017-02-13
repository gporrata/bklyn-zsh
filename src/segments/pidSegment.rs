#![allow(non_upper_case_globals)]

use ::segments::*;
use std::env;

const bg0: u32 = 0xbdc3c7;
const fg0: u32 = 0x094d77;
const fg1: u32 = 0x000000;

pub fn segment() -> Option<Vec<Part>> {
  env::var("bklyn_zsh_pid").ok()
    .and_then(|pid| pid.parse::<u32>().ok())
    .and_then(|pid| if pid == 0 { None } else { Some(pid) })
    .map(|pid|
      vec![
        Part::Bg(bg0),
        Part::Fg(fg0),
        Part::Text("\u{f12e} ".to_string()),
        Part::Fg(fg1),
        Part::Text(pid.to_string())
      ]
    )
}
