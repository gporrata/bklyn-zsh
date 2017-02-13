#![allow(non_upper_case_globals)]

extern crate futures;

use ::segments::*;
use ::segments::Segment;
use self::futures::future::*;
use std::env;

const bg0: u32 = 0xbdc3c7;
const fg0: u32 = 0x094d77;
const fg1: u32 = 0x000000;

pub fn segment() -> Segment {
  let pid = env::var("bklyn_zsh_pid").expect("Missing bklyn_zsh_pid");
  let result = vec![
    Part::Bg(bg0),
    Part::Fg(fg0),
    Part::Text(format!("\u{f12e} ")),
    Part::Fg(fg1),
    Part::Text(pid)
  ];
  ok(result).boxed()
}
