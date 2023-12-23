#![feature(array_windows)]
use std::collections::HashSet;

fn main() {
    let s = include_str!("../../input.txt")
        .lines()
        .collect::<Vec<&str>>();
    println!(
        "{}",
        (0..(s.len() / 3))
            .map(|e| (s[e * 3], s[e * 3 + 1], s[e * 3 + 2]))
            .map(|(a, b, c)| {
                let am: HashSet<char> = a.chars().collect();
                let bm: HashSet<char> = b.chars().collect();
                c.chars()
                    .filter(|e| am.contains(e) && bm.contains(e))
                    .last()
                    .expect(&format!("{a} {b} {c}"))
            })
            .map(|e| match e as i32 - 64 {
                d if d <= 26 => d + 26,
                d if d > 26 => d - 32,
                _ => 0,
            })
            .sum::<i32>(),
    );
}
