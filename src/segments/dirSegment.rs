#![allow(non_upper_case_globals)]

use std::collections::HashMap;
use ::segments::*;
use std::path::Path;
use std::env;

const bg0: u32 = 0x094d77;
const fg0: u32 = 0x33a4ea;
const fg1: u32 = 0xffffff;

type StaticDataMap = HashMap<&'static str, &'static str>; 

fn lang_icons() -> StaticDataMap {
  hashmap![
    "home" => "\u{f015}  ",
    "npm" => "\u{e71e}  ",
    "docker" => "\u{e7b0}  ",
    "java" => "\u{e738}  ",
    "cpp" => "\u{e789}  ", // actually gnu
    "rust" => "\u{f085}  ", // actually gears
    "scala" => "\u{e706}  ",
    "node" => "\u{e7b2}  ",
    "js" => "\u{e74e}  ",
    "maven" => "\u{e738}  ", // maven icon unrecognizable, use java instead "\ue7c4",
    "other" => "" // seems silly "\uf07b",
  ]
}

fn lang_markers() -> StaticDataMap {
  hashmap![
    "package.json" => "node",
    "node_modules" => "node",
    "build.sbt" => "scala",
    "pom.xml" => "maven",
    "build.xml" => "java",
    "makefile" => "cpp",
    "Cargo.toml" => "rust"
  ]
}

fn find_lang(dir: &Path, home: &Path, top: bool, langMarkers: &StaticDataMap) -> &'static str {
  if dir == home {
    if top {
      "home"
    } else {
      "other"
    }
  }
  else {
    let dirBuf = dir.join("?");
    langMarkers
      .iter()
      .find(|marker| dirBuf.with_file_name(marker.0).is_file()) 
      .map(|marker| *marker.1)
      .unwrap_or_else(||
        dir.parent()
          .map(|parent| find_lang(parent, home, false, &langMarkers))
          .unwrap_or("root")
      )
  }
}

fn find_icon(lang: &'static str, langIcons: &StaticDataMap) -> &'static str {
  langIcons.get(lang).unwrap_or(&"")
}

fn build_dir_segment(icon: &'static str, dir: String) -> Vec<Part> {
  vec![
    Part::Bg(bg0), 
    Part::Fg(fg0), 
    Part::StaticText(icon), 
    Part::Fg(fg1), 
    Part::Text(dir)
  ]
}

pub fn segment() -> Option<Vec<Part>> {
  let _pwd = env::var("PWD").expect("Missing PWD");
  let pwd = Path::new(&_pwd);
  let _home = env::var("HOME").expect("Missing HOME");
  let home = Path::new(&_home);
  let langMarkers = lang_markers();
  let lang = find_lang(pwd, home, true, &langMarkers);
  let langIcons = lang_icons();
  let icon = find_icon(&lang, &langIcons);
  Some(build_dir_segment(icon, pwd.to_string_lossy().into_owned()))

}

// for docker use e7b0
