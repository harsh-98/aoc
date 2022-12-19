// obisidan -> clay -> ore
use regex::Regex;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
pub fn solve(time: i64, take_bp: usize) -> Vec<i64> {
    let re = Regex::new(r"(\d+) (ore|clay|obsidian)").unwrap();
    let bp = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let x = s.split(":").nth(1).expect("can't split bp by :");
            let bp_ = x
                .split(".")
                .take(4)
                .map(|x| {
                    let res = re
                        .captures_iter(x)
                        .map(|cap| {
                            (
                                cap[1].parse::<i64>().expect("what err?"),
                                cap[2].to_string(),
                            )
                        })
                        .collect::<Vec<(i64, String)>>();
                    get_needed_res(res)
                })
                .collect::<Vec<(i64, i64, i64, i64)>>();
            bp_
        })
        .collect::<Vec<Vec<(i64, i64, i64, i64)>>>();

    let mut ans = Vec::<i64>::new();
    for v in bp.into_iter().take(take_bp) {
        let dp = Rc::new(RefCell::new(HashMap::new()));
        let s = Solver { bp: v };
        let geo = s.solve(1, 0, 0, time, dp.clone());
        println!("solved {geo}");
        ans.push(geo);
    }
    ans
}

struct Solver {
    bp: Vec<(i64, i64, i64, i64)>,
}

impl Solver {
    fn solve(
        &self,
        bots: i64,
        prev_next_bots: i64,
        res: i64,
        time: i64,
        dp: Rc<RefCell<HashMap<(i64, i64, i64, i64), i64>>>,
    ) -> i64 {
        // println!("{} {} {} {}", bots, prev_next_bots, res, time);
        //

        if let Some(val) = dp.borrow().get(&(bots, prev_next_bots, res, time)) {
            return *val;
        }
        let mut res_vec = get_bot(res);
        if time == 0 {
            return res_vec.3;
        }
        //
        let (o, c, ob, g) = get_bot(bots);
        //
        res_vec.0 += o;
        res_vec.1 += c;
        res_vec.2 += ob;
        res_vec.3 += g;
        //
        let mut ans = 0;
        let prev_next_bots_vec = get_bot(prev_next_bots);
        let updated_bots = (
            o + prev_next_bots_vec.0,
            c + prev_next_bots_vec.1,
            ob + prev_next_bots_vec.2,
            g + prev_next_bots_vec.3,
        );
        for (left_res, next_bot) in yield_com(&res_vec, &self.bp) {
            let x = self.solve(
                get_mask(limit_res(&updated_bots, &self.bp)),
                get_mask(next_bot),
                get_mask(limit_res(&left_res, &self.bp)),
                time - 1,
                dp.clone(),
            );
            ans = i64::max(ans, x);
        }
        dp.borrow_mut()
            .insert((bots, prev_next_bots, res, time), ans);
        ans
    }
    //
}
//

fn limit_res(
    updated_bots: &(i64, i64, i64, i64),
    bp: &Vec<(i64, i64, i64, i64)>,
) -> (i64, i64, i64, i64) {
    let limit = bp.iter().fold((0, 0, 0), |a, b| {
        (i64::max(a.0, b.0), i64::max(a.1, b.1), i64::max(a.2, b.2))
    });
    (
        if updated_bots.0 > limit.0 {
            limit.0
        } else {
            updated_bots.0
        },
        if updated_bots.1 > limit.1 {
            limit.1
        } else {
            updated_bots.1
        },
        if updated_bots.2 > limit.2 {
            limit.2
        } else {
            updated_bots.2
        },
        updated_bots.3,
    )
}
fn get_bot(n: i64) -> (i64, i64, i64, i64) {
    (
        n % 1000,
        (n / 1_000) % 1000,
        (n / 1_000_000) % 1000,
        (n / 1_000_000_000) % 1000,
    )
}
fn get_mask(res: (i64, i64, i64, i64)) -> i64 {
    res.0 + res.1 * 1000 + res.2 * 1000_000 + res.3 * 1000_000_000
}

fn get_needed_res(res: Vec<(i64, String)>) -> (i64, i64, i64, i64) {
    let mut c = (0, 0, 0, 0);
    for i in res.iter() {
        match i.1.as_ref() {
            "ore" => c.0 += i.0,
            "clay" => c.1 += i.0,
            "obsidian" => c.2 += i.0,
            "geode" => c.3 += i.0,
            _ => todo!(),
        };
    }
    c
}

//
fn non_negative_res(left_res: &(i64, i64, i64, i64)) -> bool {
    left_res.0 >= 0 && left_res.1 >= 0 && left_res.2 >= 0 && left_res.3 >= 0
}

fn how_many_can_be_created(res: &(i64, i64, i64, i64), needed: &(i64, i64, i64, i64)) -> i64 {
    let lambda = |n, d| {
        if d != 0 {
            Some(n / d)
        } else {
            None
        }
    };
    let v = [
        lambda(res.0, needed.0),
        lambda(res.1, needed.1),
        lambda(res.2, needed.2),
        lambda(res.3, needed.3),
    ];
    let created = v.iter().filter_map(|x| *x).min();
    let ans = created.unwrap_or(0);
    // println!("{ans}");
    if ans >= 1 {
        1
    } else {
        0
    }
}

fn yield_com(
    res: &(i64, i64, i64, i64),
    bp: &Vec<(i64, i64, i64, i64)>,
) -> Vec<((i64, i64, i64, i64), (i64, i64, i64, i64))> {
    let mut v = Vec::new();
    for i in 0..=how_many_can_be_created(res, &bp[0]) {
        for j in 0..=how_many_can_be_created(res, &bp[1]) {
            for k in 0..=how_many_can_be_created(res, &bp[2]) {
                for l in 0..=how_many_can_be_created(res, &bp[3]) {
                    if i + j + k + l <= 1 {
                        let next_bots = (i, j, k, l);
                        if let Some(left_res) = get_left_res(res, bp, &next_bots) {
                            v.push((left_res, next_bots));
                        }
                    }
                }
            }
        }
    }
    v
}

fn get_left_res(
    res: &(i64, i64, i64, i64),
    bp: &Vec<(i64, i64, i64, i64)>,
    next_bots: &(i64, i64, i64, i64),
) -> Option<(i64, i64, i64, i64)> {
    let mut left_res = res.clone();
    used_res(&bp[0], next_bots.0, &mut left_res);
    used_res(&bp[1], next_bots.1, &mut left_res);
    used_res(&bp[2], next_bots.2, &mut left_res);
    used_res(&bp[3], next_bots.3, &mut left_res);
    if non_negative_res(&left_res) {
        Some(left_res)
    } else {
        None
    }
}
fn used_res(
    res_per_bot: &(i64, i64, i64, i64),
    no_of_bot: i64,
    left_res: &mut (i64, i64, i64, i64),
) {
    left_res.0 -= no_of_bot * res_per_bot.0;
    left_res.1 -= no_of_bot * res_per_bot.1;
    left_res.2 -= no_of_bot * res_per_bot.2;
    left_res.3 -= no_of_bot * res_per_bot.3;
}
