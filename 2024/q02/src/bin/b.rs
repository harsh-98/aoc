fn main() {
    println!(
        "{:?}",
        include_str!("../../input.txt")
            .split("\n")
            .filter_map(|n: &str| {
                let tmp = n
                    .split_whitespace()
                    .map(|e| e.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                let mut ans = false;
                for i in 0..tmp.len() {
                    let mut x = tmp.iter().map(|&e| e).collect::<Vec<i32>>();
                    x.remove(i);
                    let a = x.iter();
                    let b = x.iter();
                    let direction = x.get(1).unwrap() - x.get(0).unwrap() > 0;
                    ans = ans
                        | (a.zip(b.skip(1))).fold(true, |folded, (a, b)| {
                            let x = (a - b).abs();
                            folded && (x > 0 && x < 4) && ((b - a > 0) == direction)
                        });
                }
                if ans {
                    Some(1)
                } else {
                    None
                }
            })
            .sum::<i32>()
    );
}
