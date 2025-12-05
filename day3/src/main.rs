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

fn get_max_jolts(s: &String) -> String {
  s.split("")
    .max()
    .unwrap_or_else(|| panic!("Unable to get max joltage!"))
    .to_owned()
}

fn calculate_joltage(bank: &str, n: usize) -> u64 {
  let mut s = bank.to_owned();
  let mut joltage = String::from("");
  for max_end in (0..=n-1).rev() {
    let l = s.len();
    let candidates = s[..l-max_end].to_owned();
    let max_in_range = &get_max_jolts(&candidates);
    joltage += max_in_range;
    let max_index = candidates.find(max_in_range.as_str())
      .unwrap_or_else(|| panic!("Didn't find max joltage in range."));
    s = s[max_index+1..].to_owned();
  }
  joltage.parse().unwrap_or_else(|_| panic!("Unable to parse joltage."))
}

fn main() {
  let Some(input) = read_input() else {
    println!("Usage: day3 -f|--file <file>");
    println!(" cat <file> | day3");
    return;
  };
  let password: u64 = input
    .split("\n")
    .map(|b| {
      let out = calculate_joltage(&b, 12);
      out
    })
    .sum();
  println!("Password: {}", password);
}
