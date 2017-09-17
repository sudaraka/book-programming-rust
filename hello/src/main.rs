use std::io::Write;
use std::str::FromStr;

fn main() {
  let
    mut numbers = Vec::new();

  for arg in std::env::args().skip(1) {
    numbers.push(
      u64::from_str(&arg).expect("Error passing argument")
    );
  }

  if 0 == numbers.len() {
    writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();

    std::process::exit(1);
  }

  let
    mut d = numbers[0];

  for m in &numbers[1..] {
    d = gcd(d, *m);
  }

  println!("The greates common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
  assert!(0 != n && 0 != m);

  while 0 != m  {
    if m < n {
      let t = m; m = n; n = t;
    }

    m = m % n;
  }

  n
}

#[test]
fn test_gcd() {
  assert_eq!(
    gcd(2 * 5 * 11 * 17, 3 * 7 * 13 * 19),
    1
  );

  assert_eq!(
    gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19),
    3 * 11
  );
}
