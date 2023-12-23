pub fn main() {
    let mut v = include_str!("../../input.txt")
        .split("\n\n")
        .map(|n: &str| {
            n.split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    v.sort_by(|a, b| b.cmp(a));
    println!("{:?}", v.iter().take(3).sum::<i32>());
}
