use std::{collections::HashMap, fmt::Display, time::Duration};

const ITERATIONS: u32 = 1000;

fn part1() -> (Box<dyn Display>, Duration) {
    let input = std::fs::read_to_string("day1/input.txt").unwrap();

    let (mut arr1, mut arr2): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut c = line.split_whitespace();
            let a = c.next().unwrap().parse::<i32>().unwrap();
            let b = c.next().unwrap().parse::<i32>().unwrap();
            (a, b)
        })
        .unzip();

    let before = std::time::Instant::now();
    arr1.sort();
    arr2.sort();

    let result = std::iter::zip(arr1, arr2)
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    dbg!(&result);

    (Box::new(result), before.elapsed())
}

fn part2() -> (Box<dyn Display>, Duration) {
    let input = std::fs::read_to_string("day1/input.txt").unwrap();

    let before = std::time::Instant::now();
    let (arr1, arr2): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut c = line.split_whitespace();
            let a = c.next().unwrap().parse::<usize>().unwrap();
            let b = c.next().unwrap().parse::<usize>().unwrap();
            (a, b)
        })
        .unzip();

    let mut similarity = HashMap::new();
    for n in arr1 {
        let count = arr2.iter().filter(|x| **x == n).count();

        if let Some(existing_score) = similarity.get(&n) {
            similarity.insert(n, existing_score + n * count);
        } else {
            similarity.insert(n, n * count);
        }
    }

    let result = similarity.values().sum::<usize>();

    dbg!(&result);

    (Box::new(result), before.elapsed())
}

fn main() {
    let average_elapsed = std::iter::repeat_with(part1)
        .take(ITERATIONS as usize)
        .map(|(_, elapsed)| {
            dbg!(elapsed);
            elapsed
        })
        .sum::<Duration>()
        / ITERATIONS;

    println!("average: {average_elapsed:?}");

    let average_elapsed = std::iter::repeat_with(part2)
        .take(ITERATIONS as usize)
        .map(|(_, elapsed)| {
            dbg!(elapsed);
            elapsed
        })
        .sum::<Duration>()
        / ITERATIONS;

    println!("average: {average_elapsed:?}");
}
