pub fn main() {
    println!(
        "{:?}",
        include_str!("../../input.txt")
            .split("\n\n")
            .map(|n: &str| {
                n.split_whitespace()
                    .map(|e| e.parse::<i32>().unwrap())
                    .sum::<i32>()
            })
            .max()
            .unwrap_or_default(),
    );
}
