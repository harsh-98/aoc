use std::collections::HashMap;
fn main() {
    let digitals    = HashMap::<&str, i32>::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("0", 0),

    ]);
    println!(
        "{:?}",
        include_str!("../input.txt")
            .split("\n")
            .map(|s| {
                let (mut min, mut max) = (s.len(), -1);
                let (mut first, mut last) = (-1, -1);
                for (k,&v) in digitals.iter() {
                    if let Some(ind) = s.find(k) {
                        if ind < min {
                            min = ind;
                            first =  v;
                        }
                        if ind as i32 > max as i32{
                            max = ind as i32;
                            last = v;
                        }
                    }
                }
                println!("{} {}", first, last);
                first*10+ last
            })
            .sum::<i32>()
    );
}
