use regex::Regex;
use std::collections::HashSet;
//
fn main() {
    let mut hs: HashSet<i32> = HashSet::new();
    let line_no = 2000000;
    let re = Regex::new(r"(-?\d+)").unwrap();
    let v = include_str!(r"../../input.txt")
        .lines()
        .map(|s| {
            let points = re
                .captures_iter(s)
                .map(|x| x[1].parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            ((points[0], points[1]), (points[2], points[3]))
        })
        .map(|(s, b)| {
            println!("{:?} {:?}", s, b);
            let distance = i32::abs(s.0 - b.0) + i32::abs(s.1 - b.1);
            let allowed = distance - i32::abs(line_no - s.1);
            if allowed >= 0 {
                for i in 0..i32::abs(allowed) + 1 {
                    hs.insert(s.0 + i);
                    hs.insert(s.0 - 1 * i);
                }
            }
            b
        })
        .collect::<Vec<(i32, i32)>>();

    v.iter().for_each(|(x, y)| {
        if *y == line_no {
            hs.remove(x);
        };
    });
    println!("{}", hs.len());
}
