use regex::Regex;
//
fn main() {
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
            let distance = i32::abs(s.0 - b.0) + i32::abs(s.1 - b.1);
            (s, distance)
        })
        .collect::<Vec<((i32, i32), i32)>>();

    let max_allowed = 4_000_000;
    for i in 0..max_allowed + 1 {
        if let Some(a) = solve(&v, i, max_allowed) {
            println!("{:?}:{}", a, a.0 as i64 * 4_000_000 + a.1 as i64);
            return;
        }
    }
}

fn solve(v: &Vec<((i32, i32), i32)>, line_no: i32, max_allowed: i32) -> Option<(i32, i32)> {
    let mut seg = v
        .iter()
        .filter_map(|(s, dis)| {
            let allowed = dis - i32::abs(line_no - s.1);
            if allowed >= 0 {
                return Some((s.0 - allowed, s.0 + allowed));
            }
            None
        })
        .collect::<Vec<(i32, i32)>>();
    seg.sort_by(|a, b| a.0.cmp(&b.0)); // sort in increasing order by min
                                       // println!("{:?}", seg);

    let mut check = 0;
    for &(min, max) in seg.iter() {
        if check > max_allowed {
            break;
        } else if check >= min {
            check = i32::max(check, max + 1);
        } else {
            break;
        }
    }
    if check > max_allowed {
        None
    } else {
        Some((check, line_no))
    }
}
