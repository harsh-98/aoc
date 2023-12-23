use day13::{compare, Cmp};
fn main() {
    let ans = include_str!("../../input.txt")
        .split("\n\n")
        .map(|s| s.split_once("\n").unwrap())
        .enumerate()
        .filter_map(|(i, (x, y))| {
            if compare(
                &x.chars().collect::<Vec<char>>()[..],
                &y.chars().collect::<Vec<char>>()[..],
            ) == Cmp::More
            {
                // println!("ans {}", i + 1);
                return Some(i + 1);
            };
            None
        })
        .sum::<usize>();
    println!("{}", ans);
}
