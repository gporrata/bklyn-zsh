
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use joiners::ColorOperations;

pub struct ZshColorOperations {}

impl ColorOperations for ZshColorOperations {
  fn all_reset(&self) -> &'static str { 
    "#[default]"
  }
  fn fg_reset(&self) -> &'static str {
    "#[fg=default]"
  }
  fn bg_reset(&self) -> &'static str {
    "#[bg=default]"
  }
  fn fg(&self, color: u32) -> String {
    format!("#[fg=#{:06x}]", color)
  }
  fn bg(&self, color: u32) -> String {
    format!("#[bg=#{:06x}]", color)
  }
}

pub fn cop() -> ZshColorOperations {
  ZshColorOperations {}
}
