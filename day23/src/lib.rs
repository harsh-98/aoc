use std::collections::{HashMap, HashSet};
pub fn solve(no_of_rounds: i32) -> (usize, i32) {
    let mut elv = HashSet::new();
    include_str!("../input.txt")
        .lines()
        .enumerate()
        .for_each(|(x, en)| {
            en.chars().enumerate().for_each(|(y, c)| {
                if c == '#' {
                    elv.insert((x as i32, y as i32));
                }
            });
        });

    let dirs: [[(i32, i32); 3]; 4] = [
        [(-1, -1), (-1, 0), (-1, 1)],
        [(1, -1), (1, 0), (1, 1)],
        [(-1, -1), (0, -1), (1, -1)],
        [(-1, 1), (0, 1), (1, 1)],
    ];

    let mut curDir = 0;
    //
    let mut rounds = 0;
    loop {
        let mut newCord = HashMap::new();
        for ele in elv.iter() {
            if !check(&elv, ele) {
                continue;
            }
            for mut dir_set_ind in 0..4 {
                dir_set_ind = (curDir + dir_set_ind) % 4;
                //
                let mut canGo = true;
                for dir in dirs[dir_set_ind].iter() {
                    let check_cord = (ele.0 + dir.0, ele.1 + dir.1);
                    if elv.contains(&check_cord) {
                        canGo = false;
                    }
                }
                if canGo {
                    let new_cord = (
                        ele.0 + dirs[dir_set_ind][1].0,
                        ele.1 + dirs[dir_set_ind][1].1,
                    );
                    let x = newCord.entry(new_cord).or_insert(vec![]);
                    x.push(*ele);
                    break;
                }
            }
        }
        curDir += 1;
        for (k, v) in newCord.iter() {
            if v.len() == 1 {
                elv.remove(&v[0]);
                elv.insert((k.0, k.1));
            }
        }
        rounds += 1;
        if rounds == no_of_rounds {
            break;
        }
        // print(&elv);
        if newCord.len() == 0 {
            break;
        }
    }
    let rec = cord(&elv);
    let ans = (rec.0 - rec.2 + 1) as usize * (rec.1 - rec.3 + 1) as usize - elv.len();
    return (ans, rounds);
}

fn cord(elv: &HashSet<(i32, i32)>) -> (i32, i32, i32, i32) {
    elv.iter().fold((0, 0, i32::MAX, i32::MAX), |a, b| {
        (
            i32::max(a.0, b.0),
            i32::max(a.1, b.1),
            i32::min(a.2, b.0),
            i32::min(a.3, b.1),
        )
    })
}
fn print(elv: &HashSet<(i32, i32)>) {
    println!("----------");
    let rec = cord(elv);
    for i in rec.2..=rec.0 {
        let mut s = String::new();
        for j in rec.3..=rec.1 {
            if elv.contains(&(i, j)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        println!("{s}");
    }
}
fn check(elv: &HashSet<(i32, i32)>, ele: &(i32, i32)) -> bool {
    let mut ans = false;
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for dir in dirs.iter() {
        let c = (ele.0 + dir.0, ele.1 + dir.1);
        ans |= elv.contains(&c);
    }
    return ans;
}
