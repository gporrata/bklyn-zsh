#![allow(non_upper_case_globals)]

extern crate futures;

use std::collections::HashMap;
use ::segments::*;
use self::futures::future::*;
use std::path::Path;
use std::env;

//#[macro_export]
macro_rules! datamap {
  ( $( $a:expr => $b:expr ),* ) => {{
    let mut _map = ::std::collections::HashMap::new();
    $(
      _map.insert($a.to_string(), $b.to_string());
    )*
    _map
  }} 
}

const bg0: u32 = 0x094d77;
const fg0: u32 = 0x33a4ea;
const fg1: u32 = 0xfffff;

// async logic behind DirSegment
struct DirSegmentLogic {
  langIcons: HashMap<String, String>,
  langMarkers: HashMap<String, String>
}

impl DirSegmentLogic {
  fn new() -> DirSegmentLogic {
    DirSegmentLogic {
      langIcons: DirSegmentLogic::lang_icons(),
      langMarkers: DirSegmentLogic::lang_markers()
    }
  }
  fn lang_icons() -> HashMap<String, String> {
    datamap![
      "home" => "\u{f015}",
      "npm" => "\u{e71e}",
      "docker" => "\u{e7b0}",
      "java" => "\u{e738}",
      "cpp" => "\u{e789}", // actually gnu
      "rust" => "\u{f085}", // actually gears
      "scala" => "\u{e706}",
      "node" => "\u{e7b2}",
      "js" => "\u{e74e}",
      "maven" => "\u{e738}", // maven icon unrecognizable, use java instead "\ue7c4",
      "other" => "" // seems silly "\uf07b",
    ]
  }
  fn lang_markers() -> HashMap<String, String> {
    datamap![
      "package.json" => "node",
      "node_modules" => "node",
      "build.sbt" => "scala",
      "pom.xml" => "maven",
      "build.xml" => "java",
      "makefile" => "cpp",
      "Cargo.toml" => "rust"
    ]
  }
  fn find_lang(&self, dir: &Path, home: &Path, top: bool) -> String {
    if dir == home {
      if top {
        "home".to_string() 
      } else {
        "other".to_string()
      }
    }
    else {
      let dirBuf = dir.join("?");
      match self.langMarkers
        .iter()
        .find(|marker| {
          dirBuf.with_file_name(marker.0).is_file()
        }) 
        .map(|marker| marker.1) {
          Some(lang) => lang.to_string(),
          None => {
            match dir.parent() {
              Some(parent) => self.find_lang(parent, home, false),
              None => "root".to_string()
            }
          }
        }
    }
  }
  fn find_icon(&self, lang: &String) -> String {
    match self.langIcons.get(lang) {
      Some(icon) => format!("{}  ", icon), 
      None => "".to_string() 
    }
  }
  fn build_dir_segment(&self, icon: String, dir: String) -> Vec<Part> {
    vec![
      Part::Bg(bg0), 
      Part::Fg(fg0), 
      Part::Text(icon), 
      Part::Fg(fg1), 
      Part::Text(dir)
    ]
  }
}

// shows the directory and type of lang used in a project
pub struct DirSegment {}

impl DirSegment {
  // creates dir segment
  pub fn new() -> DirSegment {
    DirSegment {}
  }
} 

impl Segment for DirSegment {
  fn text(&self) -> BoxFuture<Vec<Part>, ()>  {
    lazy(move || {
      let _pwd = env::var("PWD").expect("Missing PWD");
      let pwd = Path::new(&_pwd);
      let _home = env::var("HOME").expect("Missing HOME");
      let home = Path::new(&_home);
      let logic = DirSegmentLogic::new();
      let lang = logic.find_lang(pwd, home, true);
      let icon = logic.find_icon(&lang);
      let segment = logic.build_dir_segment(icon, pwd.to_string_lossy().into_owned());
      ok::<Vec<Part>, ()>(segment)
    })
    .boxed()
  }
}

// for docker use e7b0
