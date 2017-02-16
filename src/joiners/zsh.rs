#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use joiners::ColorOperations;

pub struct ZshColorOperations {}

impl ColorOperations for ZshColorOperations {
  fn all_reset(&self) -> &'static str { 
    "%{\u{1b}[0m%}"
  }
  fn fg_reset(&self) -> &'static str {
    "%{\u{1b}[39m%}" // or is it 38?
  }
  fn bg_reset(&self) -> &'static str {
    "%{\u{1b}[49m%}" //  or is it 48?
  }
  fn fg(&self, color: u32) -> String {
    let b = color & 0xff;
    let g = (color >> 8) & 0xff;
    let r = (color >> 16) & 0xff;
    format!("{}\u{1b}[38;2;{};{};{}m{}", "%{", r, g, b, "%}") 
  }
  fn bg(&self, color: u32) -> String {
    let b = color & 0xff;
    let g = (color >> 8) & 0xff;
    let r = (color >> 16) & 0xff;
    format!("{}\u{1b}[48;2;{};{};{}m{}", "%{", r, g, b, "%}") 
  }
}

pub fn cop() -> ZshColorOperations {
  ZshColorOperations {}
}
