use std::{collections::HashMap, fmt::Display, time::Duration};

const ITERATIONS: u32 = 1000;

#[derive(Debug, PartialEq, Eq)]
struct Multiplication {
    left: i32,
    right: i32,
}

fn extract_digit(s: &str) -> (&str, i32) {
    let digits_end = s
        .char_indices()
        .find_map(|(idx, c)| if !c.is_ascii_digit() { Some(idx) } else { None })
        .unwrap_or(s.len());

    let digits = &s[..digits_end];
    let remainder = &s[digits_end..];

    (remainder, digits.parse().unwrap())
}

pub fn parse_multiplication(s: &str) -> Result<(&str, (i32, i32)), &'static str> {
    let s = s.strip_prefix("mul").ok_or("expected mul")?;
    let s = s.strip_prefix('(').ok_or("expected (")?;
    let (s, left) = extract_digit(s);
    let s = s.strip_prefix(',').ok_or("expected ,")?;
    let (s, right) = extract_digit(s);
    let s = s.strip_prefix(')').ok_or("expected )")?;
    Ok((s, (left, right)))
}

pub fn parse_do(s: &str) -> Result<&str, &'static str> {
    let s = s.strip_prefix("do()").ok_or("expected do()")?;
    Ok(s)
}

pub fn parse_dont(s: &str) -> Result<&str, &'static str> {
    let s = s.strip_prefix("don't()").ok_or("expected don't()")?;
    Ok(s)
}

#[test]
fn test_extract_digit() {
    assert_eq!(extract_digit("123"), ("", 123));
    assert_eq!(extract_digit("1ab"), ("ab", 1));
}

#[test]
fn test_multiplication() {
    assert_eq!(parse_multiplication("mul(1,2)"), Ok(("", (1, 2))));
    assert_eq!(parse_multiplication("mul(1,4)"), Ok(("", (1, 4))));
    assert_eq!(parse_multiplication("mul(1,2]"), Err("expected )"));
    assert_eq!(parse_multiplication("mul(1 ,2)"), Err("expected ,"));
}

fn part1() -> (Box<dyn Display>, Duration) {
    let input = std::fs::read_to_string("day3/input.txt").unwrap();
    //let input =
    //    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
    let before = std::time::Instant::now();

    let mut enabled = true;
    let result = input
        .lines()
        .map(|line| {
            let mut s = line;
            let mut sum = 0;

            while !s.is_empty() {
                if let Ok((remainder, (left, right))) = parse_multiplication(s) {
                    if enabled {
                        println!("evaluating: mul({left}, {right})");
                        sum += left * right;
                    } else {
                        println!("skipping: mul({left}, {right})");
                    }
                    s = remainder;
                    continue;
                };

                if let Ok(remainder) = parse_do(s) {
                    println!("enabled multiplications");
                    enabled = true;
                    s = remainder;
                    continue;
                };

                if let Ok(remainder) = parse_dont(s) {
                    println!("disabled multiplications");
                    enabled = false;
                    s = remainder;
                    continue;
                };

                s = &s[1..];
            }

            sum
        })
        .sum::<i32>();

    (Box::new(result), before.elapsed())
}

fn main() {
    //let average_elapsed = std::iter::repeat_with(part1)
    //    .take(ITERATIONS as usize)
    //    .map(|(_, elapsed)| elapsed)
    //    .sum::<Duration>()
    //    / ITERATIONS;

    let (result, elapsed) = part1();
    println!("result: {result}");
    println!("elapsed: {elapsed:?}");

    //println!("average: {average_elapsed:?}");
}
