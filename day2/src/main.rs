use std::{env, fs, io::{IsTerminal, Read}};

#[derive(Debug)]
struct Range {
  start: u64,
  end: u64
}

fn read_stdin() -> Option<String> {
  if std::io::stdin().is_terminal() {
    return None;
  }
  let mut buf = String::new();
  std::io::stdin().read_to_string(&mut buf).ok()?;
  Some(buf.trim().to_owned())
}

fn parse_args() -> Option<String> {
  let mut args = env::args().skip(1);
  let flag = args.next()?;
  if !matches!(flag.as_str(), "-f" | "--file") {
    return None
  }
  args.next()
}

fn read_file(path: &str) -> Option<String> {
  fs::read_to_string(path).ok().map(|s| s.trim().to_owned())
}

fn read_input() -> Option<String> {
  if let Some(input) = read_stdin() {
    return Some(input);
  }
  let path = parse_args()?;
  read_file(&path)
}

fn get_ranges(ranges: &str) -> Vec<Range> {
  ranges.split(',').map(|r| {
    let (start_str, end_str) = r.split_once('-')
      .unwrap_or_else(|| panic!("Expected range, got: {}", r));
    let start: u64;
    let end: u64;
    start = start_str.parse()
      .unwrap_or_else(|_| panic!("Failed parsing start of range: {}", start_str));
    end = end_str.parse()
      .unwrap_or_else(|_| panic!("Failed parsing end of range: {}", end_str));
    Range{ start: start, end: end }
  }).collect()
}

// checks if id is valid
// invalid:
//  - starting with '0'
//  - patterns repeats twice
//   * 11
//   * 1212
//   * 123123
// valid:
//  - 1231231
fn is_id_valid_part_one(id: &u64) -> bool {
  let s = id.to_string();
  if s.starts_with("0") {
    return false;
  }
  let len = s.len();
  // if number of characters in id is odd, the id is valid
  if len % 2 != 0 {
    return true;
  }
  // is valid if first half differs from second half
  let (first, second) = s.split_at(len/2);
  first != second
}

// checks if id is valid
// invalid:
//  - starting with '0'
//  - patterns repeats
//   * 11
//   * 1212
//   * 123123123
// valid:
//  - 1231231
fn is_id_valid_part_two(id: &u64) -> bool {
  let s = id.to_string();
  if s.starts_with("0") {
    return false;
  }
  let n = s.len();
  for l in 1..=n/2 {
    // if str len is not divisible by pattern length we can't will s with repeats
    if n%l != 0 {
      continue;
    }
    // we can skip the first l characters
    let mut i = l;
    let pattern = &s[..l];
    let mut is_repeat = true;
    while i < n {
      if &s[i..i+l] != pattern {
        is_repeat = false;
        break;
      }
      i += l;
    }
    if is_repeat {
      return false;
    }
  }
  true
}

// sum all invalid ids in range
fn sum_invalid_ids(range: &Range) -> u64 {
  (range.start..=range.end)
    .filter(|id| !is_id_valid_part_two(&id))
    .sum()
}

fn main() {
  let Some(input) = read_input() else {
    println!("Usage: day2 -f|--file <file>");
    println!("  cat <file> | day2");
    return;
  };
  let mut password: u64 = 0;
  for r in get_ranges(&input) {
    password += sum_invalid_ids(&r);
  }
  println!("Password: {}", password);
}
