fn main() {
    let total = include_str!("../../input.txt")
        .lines()
        .map(|s| day25::to10FromStr(s))
        .sum::<i64>();
    println!("{total}: {}", day25::to5FromNum(total));
}
