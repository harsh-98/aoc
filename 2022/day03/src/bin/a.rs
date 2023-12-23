use std::collections::HashSet;
fn main() {
    let t: i32 = include_str!("../../input.txt")
        .lines()
        .map(|n| (&n[..n.len() / 2], &n[n.len() / 2..]))
        .map(|(a, b)| {
            let x: HashSet<char> = a.chars().collect();
            let x = b.chars().filter(|e| x.contains(e)).last().unwrap();
            let v = match x as i32 - 64 {
                d if d <= 26 => d + 26,
                d if d > 26 => d - 32,
                _ => 0,
            };
            println!("common:{} int:{}", x, v);
            v
        })
        .sum();
    println!("{}", t);
}
