#![allow(non_upper_case_globals)]

/* example of: git -c color.status=false status --porcelain=2 --branch

# branch.oid eb744800f30f6b55147fa2c3a2f3e2591f0ff2a3
# branch.head feature/revamp-using-node
# branch.upstream origin/feature/revamp-using-node
# branch.ab +0 -0
1 .M N... 100644 100644 100644 6015922c0254ff356df8f9f5e853c65753483afa 6015922c0254ff356df8f9f5e853c65753483afa bklyn-zsh.plugin.zsh
1 .M N... 100644 100644 100644 f9d35da32a052ac459972c4fd8c669166f43a3eb f9d35da32a052ac459972c4fd8c669166f43a3eb src/bklyn-zsh.js
1 .M N... 100644 100644 100644 93069613d3b23489b727033073121fa29681995f 93069613d3b23489b727033073121fa29681995f src/dirIcon.js
1 .M N... 100644 100644 100644 81b8b72531396449eb7ff26f8aeeec243d54bb8e 81b8b72531396449eb7ff26f8aeeec243d54bb8e src/icons.js
1 .M N... 100644 100644 100644 00a3532ff48684c0a790b3654fe0523521edee63 00a3532ff48684c0a790b3654fe0523521edee63 src/osIcon.js
? hi
? src/gitStatusOf.js

----------

^branch.head (\w+) => branch name
^branch.ab +(\d+) -(\d+) => num commits ahead & behind
^? => file with untracked changes
^(\d+) .(\w) => staged changes
^(\d+) (\w). => unstaged changes
? => untracked file (will be considered to be unstaged change)
*/

extern crate regex;

use ::segments::*;
use std::process::Command;
use self::regex::*;

const bg0: u32 = 0x33d1a;
const fg0: u32 = 0x27ae60;
const fg1: u32 = 0xffffff;
//const fg2: u32 = 0xf1c40f; // not used
const fgUnstaged: u32 = 0xe74c3c;
const fgStaged: u32 = 0xe67e22;

const githubIcon: &'static str = "\u{f113}";
const branchIcon: &'static str = "\u{f126}";
const upIcon: &'static str = "\u{f0aa}";
const downIcon: &'static str = "\u{f0ab}";
const stagedIcon: &'static str = "\u{f069}";
const unstagedIcon: &'static str = "\u{f06a}";
const stashesIcon: &'static str = "\u{f01c}";

struct GitResult {
  branch: String, 
  ahead: i32,
  behind: i32,
  staged: bool,
  unstaged: bool,
}

fn git() -> Option<GitResult> {
  Command::new("git" )
    .args(&["-c","color.status=false","status","--porcelain=2","--branch"])
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
    .map(|out| {
      // find branch name
      let branchRex = RegexBuilder::new(r"^# branch.head (.+)$")
        .multi_line(true).build().unwrap();
      let branch = match branchRex.captures(&out) {
        Some(captures) => captures.get(1).map(|m| m.as_str()).unwrap_or(""),
        None => ""
      };
      // find ahead | behind counts
      let abRex = RegexBuilder::new(r"^# branch.ab \+(\d+) \-(\d+)$")
        .multi_line(true).build().unwrap();
      let (ahead, behind) = match abRex.captures(&out) {
        Some(captures) => 
          (captures.get(1).and_then(|m| m.as_str().parse::<i32>().ok()).unwrap_or(0),
          captures.get(2).and_then(|m| m.as_str().parse::<i32>().ok()).unwrap_or(0)),
        None => (0, 0)
      };
      // find if has unstaged file otherwise if has staged files
      let stagedRex = RegexBuilder::new(r"^((\?)|(\d+ ((\.)|(\w))))")
        .multi_line(true).build().unwrap();
      let mut staged = false;
      let mut unstaged = false;
      for cap in stagedRex.captures_iter(&out) {
        if !unstaged && (cap.get(2).is_some() || cap.get(5).is_some()) {
          unstaged = true;
        }
        else if !unstaged || !staged {
          staged = true;
        }
      };
      GitResult {
        branch: String::from(branch),
        ahead: ahead,
        behind: behind,
        staged: staged,
        unstaged: unstaged
      } 
    })
}

fn git_stashes() -> usize {
  Command::new("git")
    .args(&["stash","list"])
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
    .map(|out| out.split("\n").count()-1)
    .unwrap_or(0)
}

pub fn segment() -> Option<Vec<Part>> {
  git().map(|gitr|{
    let stashes = git_stashes();
    let mut result = Vec::with_capacity(100);
    result.push(Part::Bg(bg0));
    result.push(Part::Fg(fg0));
    result.push(Part::Text(format!["{}  {} ", githubIcon, branchIcon]));
    if gitr.unstaged {
      result.push(Part::Fg(fgUnstaged));
      result.push(Part::Text(format!["{} ", unstagedIcon]));
    }
    else if gitr.staged {
      result.push(Part::Fg(fgStaged));
      result.push(Part::Text(format!["{} ", stagedIcon]));
    }
    result.push(Part::Fg(fg1));
    result.push(Part::Text(gitr.branch));
    result.push(Part::Fg(fg0));
    if gitr.ahead > 0 {
      result.push(Part::Text(format![" {} ", upIcon]));
      result.push(Part::Fg(fg1));
      result.push(Part::Text(gitr.ahead.to_string()));
      result.push(Part::Fg(fg0));
    }
    if gitr.behind > 0 {
      result.push(Part::Text(format![" {} ", downIcon]));
      result.push(Part::Fg(fg1));
      result.push(Part::Text(gitr.behind.to_string()));
      result.push(Part::Fg(fg0));
    }
    if stashes > 0 {
      result.push(Part::Text(format![" {} ", stashesIcon]));
      result.push(Part::Fg(fg1));
      result.push(Part::Text(stashes.to_string()));
      result.push(Part::Fg(fg0));
    }
    result
  })
}


