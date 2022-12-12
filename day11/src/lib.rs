use std::ops::Deref;

use regex::Regex;

pub fn solve(div_by_3: bool, rounds: usize) -> usize {
    let mut monkeys = Vec::new();
    let re = Regex::new(r"\d+").unwrap();
    let sign_re = Regex::new(r"[\+\*]|old|\d+").unwrap();
    include_str!("../input.txt").split("\n\n").for_each(|d| {
        let d = d.lines().skip(1).collect::<Vec<&str>>();
        let worries = re
            .captures_iter(d[0])
            .map(|s| s[0].parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let div_by = re.captures_iter(d[2]).nth(0).unwrap()[0]
            .parse::<i64>()
            .unwrap();

        let s = sign_re
            .captures_iter(d[1])
            .map(|s| s[0].to_string())
            .collect();
        //
        monkeys.push(Monkey {
            worries: worries,
            op: s,
            div_by: div_by,
            t: d[3].chars().last().unwrap() as usize - '0' as usize,
            f: d[4].chars().last().unwrap() as usize - '0' as usize,
        });
    });

    // monkeys.iter().enumerate().for_each(|(ind, e)| {
    //     println!("{:?}", e.worries);
    // });

    let mut div_by: i64 = monkeys.iter().map(|s| s.div_by).product();
    if div_by_3 {
        div_by = 3;
    }

    //
    let mut mon_total: Vec<usize> = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut t_item = Vec::new();
            let mut f_item = Vec::new();
            mon_total[i] = mon_total[i] + monkeys[i].worries.len();
            monkeys[i].worries.iter().for_each(|&w| {
                let mut w = match (monkeys[i].op[1].deref(), monkeys[i].op[2].deref()) {
                    ("*", "old") => w * w,
                    ("*", num) => {
                        let num = num.parse::<i64>().unwrap();
                        num * w
                    }
                    ("+", "old") => w + w,
                    ("+", num) => {
                        let num = num.parse::<i64>().unwrap();
                        num + w
                    }
                    (_, _) => todo!(),
                };
                if div_by_3 {
                    w = w / 3;
                } else {
                    w = w % div_by;
                }
                if w % monkeys[i].div_by == 0 {
                    t_item.push(w);
                } else {
                    f_item.push(w);
                }
            });
            monkeys[i].worries = Vec::new();
            let t = monkeys[i].t;
            monkeys[t].worries.extend(t_item);
            let f = monkeys[i].f;
            monkeys[f].worries.extend(f_item);
        }
    }
    println!("total: {:?}", mon_total);
    mon_total.sort();
    let business = mon_total
        .iter()
        .rev()
        .take(2)
        .fold(1, |product, &e| product * e);
    business
}

#[derive(Debug)]
enum Op {
    Mul,
    Add,
}

#[derive(Debug)]
struct Monkey {
    worries: Vec<i64>,
    op: Vec<String>,
    div_by: i64,
    t: usize, // ind if the monkey in array
    f: usize, // ind if the monkey in array
}
