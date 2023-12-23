use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Dir {
    Right,
    Down,
    Left,
    Up,
}

struct Enter {
    right: Option<(usize, Dir)>,
    left: Option<(usize, Dir)>,
    up: Option<(usize, Dir)>,
    down: Option<(usize, Dir)>,
}

pub fn solve(new_cord: Box<fn(&Vec<Vec<char>>, Dir, &(usize, usize)) -> ((usize, usize), Dir)>) {
    let (puzzle, moves) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let mut start = (0, 0);
    let mut isStartSet = false;
    let matrix = puzzle
        .lines()
        .map(|s| {
            s.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == '.' && !isStartSet {
                        isStartSet = true;
                        start.1 = col;
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut count = String::new();
    let mut dir = Dir::Right;

    for step in moves.chars() {
        match step {
            '0'..='9' => {
                count.push(step);
            }
            _ => {
                let m = count.parse::<usize>().unwrap();
                count.clear();
                for _ in 0..m {
                    if !solve_for_dir(&matrix, &mut start, &mut dir, &new_cord) {
                        break;
                    }
                }

                dir = match (step, dir) {
                    ('R', Dir::Up) => Dir::Right,
                    ('L', Dir::Up) => Dir::Left,
                    ('R', Dir::Down) => Dir::Left,
                    ('L', Dir::Down) => Dir::Right,
                    ('R', Dir::Right) => Dir::Down,
                    ('L', Dir::Right) => Dir::Up,
                    ('R', Dir::Left) => Dir::Up,
                    ('L', Dir::Left) => Dir::Down,
                    _ => todo!(),
                };
            }
        }
    }
    let m = count.parse::<usize>().unwrap();
    count.clear();
    for _ in 0..m {
        solve_for_dir(&matrix, &mut start, &mut dir, &new_cord);
    }
    println!(
        "{}",
        (start.0 + 1) * 1000 + 4 * (start.1 + 1) + dir as usize
    );
}

fn valid(matrix: &Vec<Vec<char>>, cord: &(i32, i32)) -> bool {
    if cord.0 >= 0 && cord.0 < matrix.len() as i32 {
        let col = matrix[cord.0 as usize].len();
        return cord.1 >= 0 && cord.1 < col as i32;
    }
    return false;
}

fn solve_for_dir(
    matrix: &Vec<Vec<char>>,
    start: &mut (usize, usize),
    dir: &mut Dir,
    new_cord: &Box<fn(&Vec<Vec<char>>, Dir, &(usize, usize)) -> ((usize, usize), Dir)>,
) -> bool {
    let step: (i32, i32) = match dir {
        Dir::Up => (-1, 0),
        Dir::Right => (0, 1),
        Dir::Left => (0, -1),
        Dir::Down => (1, 0),
    };
    let new_c = (start.0 as i32 + step.0, start.1 as i32 + step.1);
    // println!("{new_c:?}");
    if valid(&matrix, &new_c) {
        match matrix[new_c.0 as usize][new_c.1 as usize] {
            '.' => {
                start.0 = new_c.0 as usize;
                start.1 = new_c.1 as usize;
            }
            '#' => return false,
            ' ' => {
                let new_c = new_cord(&matrix, dir.clone(), &start);
                if matrix[new_c.0 .0][new_c.0 .1] != '#' {
                    println!("{dir:?} {start:?}: {:?} {:?}", new_c.0, new_c.1);
                    *start = new_c.0;
                    *dir = new_c.1;
                } else {
                    return false;
                }
            }
            _ => todo!(),
        }
    } else {
        let new_c = new_cord(&matrix, dir.clone(), &start);
        if matrix[new_c.0 .0][new_c.0 .1] != '#' {
            println!("{dir:?} {start:?}: {:?} {:?}", new_c.0, new_c.1);
            *start = new_c.0;
            *dir = new_c.1;
        } else {
            return false;
        }
    }
    return true;
}
