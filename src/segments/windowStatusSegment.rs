#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use ::segments::*;
use ::joiners::getTmuxWindowStatusEnv;

// active
pub const bg0_active: u32 = 0xd35400;
const fg0_active: u32 = 0x0f3247;
const fg1_active: u32 = 0xffffff;

// inactive
//pub const bg0_inactive: u32 = 0x094d77;
pub const bg0_inactive: u32 = 0x0f3247;
const fg0_inactive: u32 = 0xd35400;
const fg1_inactive: u32 = 0xcccccc;
const fg2_inactive: u32 = 0x999999;

// icons
const active: &'static str = "\u{f055} ";
const inactive: &'static str = "\u{f056} ";
const extra: &'static str = " \u{f045} ";

// window:<text>
pub fn segment(text: &str) -> Option<Vec<Part>> {
  let mut textSplit = text.splitn(3, ":").collect::<Vec<&str>>();
  textSplit.reverse();
  textSplit.pop();
  let title = textSplit.pop().unwrap();
  let dir = textSplit.pop().unwrap_or("");
  let windowStatus = getTmuxWindowStatusEnv();
  let vec = if windowStatus.isActive {
    vec![
      Part::Bg(bg0_active),
      Part::Fg(fg0_active),
      Part::StaticText(active),
      Part::Fg(fg1_active),
      Part::Text(String::from(title)),
      Part::Fg(fg0_active),
      Part::StaticText(extra),
      Part::Text(String::from(dir))
    ]
  }
  else {
    vec![
      Part::Bg(bg0_inactive),
      Part::Fg(fg0_inactive),
      Part::StaticText(inactive),
      Part::Fg(fg1_inactive),
      Part::Text(String::from(title)),
      Part::Fg(fg0_inactive),
      Part::StaticText(extra),
      Part::Fg(fg2_inactive),
      Part::Text(String::from(dir))
    ]
  };
  Some(vec)
}
