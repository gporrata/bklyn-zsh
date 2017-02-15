extern crate regex;

use self::regex::Regex;
use ::segments::*;

// allows arbitrary text for a segment
// fg set with ±fg{XXXXXX} where XXXXXX is hex but also 'default'
// bg set with ±bg{XXXXXX} ''
// bg must be the first thing set for rendering separators

pub fn segment(text: &str) -> Option<Vec<Part>> {
  let color = Regex::new(r"±(fg|bg)\{([0-9a-f]{6}|default)}").unwrap();
  let mut endOfLastMatch = 0;
  let mut parts = color.captures_iter(text)
    .flat_map(|capture| {
      let colorFgBg = capture.get(1).map(|str| str.as_str()).unwrap();
      let code = capture.get(2).map(|str| str.as_str()).unwrap();
      let part = match (colorFgBg, code) { 
        ("fg", "default") => Part::FgReset {},
        ("fg", color) => Part::Fg(u32::from_str_radix(color, 16).unwrap()),
        ("bg", "default") => Part::BgReset {},
        ("bg", color) => Part::Bg(u32::from_str_radix(color, 16).unwrap()),
        _ => unimplemented!() // should never happen cause of the regex above
      };
      let currMatch = capture.get(0).unwrap();
      let start = currMatch.start();
      if start > endOfLastMatch {
        let slice = text[endOfLastMatch .. start].to_string();
        let text = Part::Text(slice);
        endOfLastMatch = currMatch.end();
        vec![text, part]
      }
      else {
        endOfLastMatch = currMatch.end();
        vec![part]
      }
    })
    .collect::<Vec<Part>>();
  if parts.is_empty() {
    None
  }
  else {
    match parts[0] {
      Part::Bg(_) => {
        if endOfLastMatch < text.len() {
          let slice = text[endOfLastMatch .. text.len()].to_string();
          let endPart = Part::Text(slice);
          parts.push(endPart);
        };
        Some(parts)
      },
      _ => None
    }
  }
}
