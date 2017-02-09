use std::collections::HashMap;
use ::segments::{Segment,Environment,Part};

struct OsSegment {
  text: Vec<Part> 
}

impl OsSegment {
  fn new(env: &HashMap<String, String>) -> OsSegment {
    let text: Vec<Part> = env.get("OSTYPE")
      .map_or_else(
        || {
          Vec::new() // unknown OS?
        },
        |os| {        
          if os == "linux-gnu" {
            // TODO: determine linux distro
            // redhat: "\u{f309}"
            // ubuntu: "\u{fe73a}"
            vec!(Part::Text(String::from("\u{f17c}")))
          }
          else if os.starts_with("darwin") {
            // TODO: show macos version maybe?
            vec!(Part::Text(String::from("\u{f179}")))
          }
          else if os == "cygwin" {
            vec!(Part::Text(String::from("\u{f17a}"))) // TODO: eval icon here; generic win icon
          }
          else if os == "msys" {
            vec!(Part::Text(String::from("\u{f17a}"))) // TODO: eval icon here; generic win icon
          }
          else if os == "win32" {
            vec!(Part::Text(String::from("\u{f17a}"))) // TODO: eval icon here; generic win icon
          }
          else if os.starts_with("freebsd") {
            vec!(Part::Text(String::from(""))) // TODO: eval icon here
          }
          else {
            Vec::new() // unknown OS?
          }
        }
      );
    OsSegment {
      text: text
    }
  }
}

impl Segment for OsSegment {
  fn get_text(&self, env: &Environment) -> &Vec<Part> {
    &self.text
  }
}
