use gcd;
use std::collections::{HashSet, VecDeque};
//
// took help from other:
// for lcm
// part 2
// that we don't need to check blizzard in the starting point
//
// in part 1
// another fix found by me was to use rem_euclid as negative points will not be in set and check_empty will return true
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Dir {
    Up,
    Down,
    Right,
    Left,
}

pub fn solve() -> (i32, i32, HashSet<(i32, i32, Dir)>) {
    let mut x_max = 0;
    let mut y_max = 0;
    let mut set = HashSet::new();
    include_str!("../input.txt")
        .lines()
        .enumerate()
        .for_each(|(x, row)| {
            x_max = x as i32 + 1;
            y_max = row.len() as i32;
            row.chars().enumerate().for_each(|(y, c)| {
                if c == '.' || c == '#' {
                    return;
                }
                let d = match c {
                    '>' => Dir::Right,
                    'v' => Dir::Down,
                    '<' => Dir::Left,
                    '^' => Dir::Up,
                    _ => todo!(),
                };
                set.insert((x as i32 - 1, y as i32 - 1, d));
            });
        });

    x_max -= 2;
    y_max -= 2;

    return (x_max, y_max, set);
}

pub fn time_it(
    x_max: i32,
    y_max: i32,
    set: &HashSet<(i32, i32, Dir)>,
    start: (i32, i32, i32),
    end: (i32, i32),
) -> i32 {
    // took help for lcm
    let lcm = x_max * y_max / gcd::binary_usize(x_max as usize, y_max as usize) as i32;
    //
    let mut seen = HashSet::new();
    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
    q.push_back(start); // x,y, time

    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)];
    while !q.is_empty() {
        let cord = q.pop_front().unwrap();
        let time = cord.2 + 1;
        //
        for dir in dirs.iter() {
            let new_cord = (dir.0 + cord.0, dir.1 + cord.1);
            if new_cord.0 == end.0 && new_cord.1 == end.1 {
                println!("{new_cord:?} {time}");
                return time;
            }
            if check(x_max, y_max, &new_cord)
                && (check_empty(x_max, y_max, time, &new_cord, &set)
                // that we don't need to check blizzar in the starting point
                    || (start.0, start.1) == (new_cord.0, new_cord.1))
            {
                let mut new_state = (new_cord.0, new_cord.1, time % lcm);
                if !seen.contains(&new_state) {
                    seen.insert(new_state.clone());
                    new_state.2 = time;
                    q.push_back(new_state);
                }
            }
        }
    }
    todo!();
}
fn check(x_max: i32, y_max: i32, new_cord: &(i32, i32)) -> bool {
    new_cord == &(-1, 0)
        || new_cord == &(x_max, y_max - 1)
        || (new_cord.0 < x_max && new_cord.0 >= 0 && new_cord.1 >= 0 && new_cord.1 < y_max)
}

const DIRS: [(i32, i32, Dir); 4] = [
    (0, 1, Dir::Left),
    (0, -1, Dir::Right),
    (1, 0, Dir::Up),
    (-1, 0, Dir::Down),
];
fn check_empty(
    x_max: i32,
    y_max: i32,
    r: i32,
    new_cord: &(i32, i32),
    set: &HashSet<(i32, i32, Dir)>,
) -> bool {
    for dir in DIRS.iter() {
        let tmp_cord = (
            vor_cord(dir.0 * r + new_cord.0, x_max),
            vor_cord(dir.1 * r + new_cord.1, y_max),
            dir.2.clone(),
        );
        if set.contains(&tmp_cord) {
            return false;
        }
    }
    true
}

fn vor_cord(x: i32, x_max: i32) -> i32 {
    x.rem_euclid(x_max) //
}
