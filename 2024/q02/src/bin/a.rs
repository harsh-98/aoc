fn main() {
    println!(
        "{:?}",
        include_str!("../../input.txt")
            .split("\n")
            .filter_map(|n: &str| {
                let x = n
                    .split_whitespace()
                    .map(|e| e.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                let a = x.iter();
                let b = x.iter();
                let direction = x.get(1).unwrap() - x.get(0).unwrap() > 0;
                if (a.zip(b.skip(1))).fold(true, |folded, (a, b)| {
                    let x = (a - b).abs();
                    folded && (x > 0 && x < 4) && ((b - a > 0) == direction)
                }) {
                    Some(1)
                } else {
                    None
                }
            })
            .sum::<i32>()
    );
}
