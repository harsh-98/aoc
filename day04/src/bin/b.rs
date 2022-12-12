fn main() {
    println!(
        "{}",
        include_str!("../../input.txt")
            .lines()
            .map(|s| s.split_once(",").unwrap())
            .map(|(a, b)| (a.split_once("-").unwrap(), b.split_once("-").unwrap()))
            .filter(|((a, b), (x, y))| {
                let a = a.parse::<i32>().unwrap();
                let b = b.parse::<i32>().unwrap();
                let x = x.parse::<i32>().unwrap();
                let y = y.parse::<i32>().unwrap();
                (a <= x && x <= b) || (a <= y && y <= b) || (x <= a && a <= y) || (x <= b && b <= y)
            })
            .count()
    );
}
