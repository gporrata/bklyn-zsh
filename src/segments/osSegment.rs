#![allow(non_upper_case_globals)]

use ::segments::*;
use std::env;
use std::process::Command;

const bg0: u32 = 0x094d77;
const fg0: u32 = 0x2ecc71;

fn wrap_icon(icon: &'static str) -> Option<Vec<Part>> {
  Some(vec![Part::Bg(bg0), Part::Fg(fg0), Part::StaticText(icon)])
}

pub fn segment() -> Option<Vec<Part>> {
  let os = env::var("OSTYPE").expect("Missing OSTYPE!");
  if os == "linux-gnu" {
    let distro = Command::new("lsb_release")
      .args(&["-si"])
      .output()
      .ok()
      .and_then(|output| {
        if output.status.success() {
          String::from_utf8(output.stdout).ok()
        }
        else {
          None
        }
      })
      .and_then(|stdout| {
        let tstdout = stdout.trim();
        if tstdout == "Ubuntu" {
          println!("Matched to ubuntu?");
          Some("\u{e73a} ")
        }
        else if tstdout == "Redhat" {
          Some("\u{e7bb} ")
        }
        else {
          None
        }
      })
      .unwrap_or("\u{f17c} ");
    wrap_icon(distro)
  }
  else if os.starts_with("darwin") {
    wrap_icon("\u{f179} ")
  }
  else if os == "cygwin" {
    wrap_icon("\u{f17a} ") // TODO: eval icon here; generic win icon
  }
  else if os == "msys" {
    wrap_icon("\u{f17a} ") // TODO: eval icon here; generic win icon
  }
  else if os == "win32" {
    wrap_icon("\u{f17a} ") // TODO: eval icon here; generic win icon
  }
  else if os.starts_with("freebsd") {
    wrap_icon("") // TODO: eval icon here
  }
  else {
    None // unknown OS?
  }
}

