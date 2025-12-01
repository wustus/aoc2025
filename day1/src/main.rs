use std::{fs, io::{IsTerminal, Read}};

enum Direction {
  Left,
  Right
}

fn read_stdin() -> Option<String> {
  if std::io::stdin().is_terminal() {
    return None
  }
  let mut buf = String::new();
  std::io::stdin().read_to_string(&mut buf).ok()?;
  Some(buf.trim().to_owned())
}

fn parse_path() -> Option<String> {
  // skip program name
  let mut args = std::env::args().skip(1);
  let flag = args.next()?;
  if !matches!(flag.as_str(), "-f" | "--file") {
    return None;
  }
  args.next()
}

fn read_input() -> Option<String> {
  if let Some(stdin) = read_stdin() {
    return Some(stdin);
  };
  let path = parse_path()?;
  let buf = fs::read_to_string(path).ok()?;
  Some(buf.trim().to_owned())
}

fn step(pos: u16, action: &str) -> u16 {
  if action.len() < 2 {
    panic!("Input wrong. Expected direction and number, got: {}", action)
  }
  let (dir_str, num_str) = action.split_at(1);
  let dir = match dir_str {
    "L" => Direction::Left,
    "R" => Direction::Right,
    _ => panic!("Input wrong. Expected direction, got: {}", dir_str),
  };
  let n: u16 = num_str
    .parse()
    .unwrap_or_else(|_| panic!("Input wrong. Expected number, got: {}", num_str));
  let new_pos = match dir {
    Direction::Left => (pos + 100 - (n % 100)) % 100,
    Direction::Right => (pos + n) % 100,
  };
  new_pos
}

fn main() {
  let Some(input) = read_input() else {
    println!("Usage: day1 -f|--file <file>");
    println!("  cat <file> | day1");
    return;
  };
  let mut pos = 50;
  let mut password: u16 = 0;
  for action in input.lines() {
    pos = step(pos, action);
    if pos == 0u16 {
      password += 1;
    }
  }
  println!("Password: {}", password);
}
