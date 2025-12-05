use std::io::{IsTerminal, Read};


fn read_stdin() -> Option<String> {
  if std::io::stdin().is_terminal() {
    return None;
  }
  let mut buf = String::new();
  std::io::stdin().read_to_string(&mut buf).ok()?;
  Some(buf.trim().to_owned())
}

fn read_file(path: &str) -> Option<String> {
  std::fs::read_to_string(path).ok()
    .map(|s| s.trim().to_owned())
}

fn parse_args() -> Option<String> {
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
  }
  let path = parse_args()?;
  read_file(&path)
}

fn calculate_joltage(bank: &str, n: usize) -> u64 {
  let bytes = bank.as_bytes();
  let mut start = 0;
  let mut joltage = String::with_capacity(n);
  for min_remaining in (0..n).rev() {
    // we have to leave at least `min_remaining` digits for future iterations
    //  the last byte we can consider is bytes.len()-1-min_remaining
    let search_end = bytes.len() - min_remaining;
    let mut max_idx = start;
    let mut max_id = bytes[start];
    for i in (start+1)..search_end {
      if bytes[i] > max_id {
        max_idx = i;
        max_id = bytes[i];
      }
    }
    start = max_idx+1;
    joltage.push(max_id as char);
  }
  joltage.parse().expect("Unable to parse joltage.")
}

fn main() {
  let Some(input) = read_input() else {
    println!("Usage: day3 -f|--file <file>");
    println!(" cat <file> | day3");
    return;
  };
  let password: u64 = input
    .lines()
    .filter(|l| !l.is_empty())
    .map(|b| calculate_joltage(&b, 12))
    .sum();
  println!("Password: {}", password);
}
