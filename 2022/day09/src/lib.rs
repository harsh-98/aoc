#![feature(array_windows)]
use std::collections::HashMap;
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Cord(pub i32, pub i32);

pub fn solve(points: &mut Vec<Cord>) {
    let mut cover: HashMap<Cord, bool> = HashMap::new();
    include_str!("../input.txt")
        .lines()
        .map(|s| s.split_once(" ").unwrap())
        .for_each(|(dir, num)| {
            let num = num.parse::<usize>().unwrap();
            match dir {
                "D" => {
                    for _ in 0..num {
                        let H = &points[0];
                        points[0] = Cord(H.0 - 1, H.1);
                        iter(&mut cover, points);
                        // print_m(&points);
                    }
                }
                "U" => {
                    for _ in 0..num {
                        let H = &points[0];
                        points[0] = Cord(H.0 + 1, H.1);
                        iter(&mut cover, points);
                        // print_m(&points);
                    }
                }
                "R" => {
                    for _ in 0..num {
                        let H = &points[0];
                        points[0] = Cord(H.0, H.1 + 1);
                        iter(&mut cover, points);
                        // print_m(&points);
                    }
                }
                "L" => {
                    for _ in 0..num {
                        let H = &points[0];
                        points[0] = Cord(H.0, H.1 - 1);
                        iter(&mut cover, points);
                        // print_m(&points);
                    }
                }
                _ => todo!(),
            }
        });
    println!("{}", cover.len());
}

fn iter(cover: &mut HashMap<Cord, bool>, points: &mut Vec<Cord>) {
    for i in 0..points.len() - 1 {
        let t = check(&points[i], &points[i + 1]);
        points[i + 1] = t;
    }
    let t = points.last().unwrap();
    // println!("{:?}", t);
    cover.insert(t.clone(), true);
}
fn check(H: &Cord, T: &Cord) -> Cord {
    let diff = i32::abs(H.0 - T.0) + i32::abs(H.1 - T.1);
    if H.0 == T.0 || T.1 == H.1 {
        if diff > 1 {
            return Cord((T.0 + H.0) / 2, (T.1 + H.1) / 2);
        }
    } else {
        if diff > 2 {
            // meaning different rows/cols and no of the cordinate 's diff is 2.
            if i32::abs(T.0 - H.0) == 2 {
                if i32::abs(T.1 - H.1) == 2 {
                    return Cord((T.0 + H.0) / 2, (T.1 + H.1) / 2);
                }
                return Cord((T.0 + H.0) / 2, H.1);
            } else {
                return Cord(H.0, (T.1 + H.1) / 2);
            }
        }
    }
    return T.clone();
}

fn print_m(p: &Vec<Cord>) {
    println!("##########");
    let mut m = vec![vec![11; 6]; 5];
    for (e, i) in p.iter().enumerate() {
        m[i.0 as usize][i.1 as usize] = i32::min(e as i32, m[i.0 as usize][i.1 as usize]);
    }

    m.iter().rev().for_each(|e| {
        e.iter().for_each(|&xx| {
            if xx != 11 {
                print!("{}", xx);
            } else {
                print!(".");
            }
        });
        println!("");
    })
}
