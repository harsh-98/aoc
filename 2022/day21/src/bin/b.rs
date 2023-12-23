use std::collections::{HashMap, VecDeque};
fn main() {
    let (dependent, ops, mut ans) = day21::get_data();

    ans.remove("humn");
    for (k, v) in ops.iter() {
        if v.len() == 1 {
            day21::calc_eles(k, &ops, &dependent, &mut ans);
        }
    }
    solve_for_root(&ops, &mut ans);

    println!("{}", ans.get("humn").unwrap())
}

fn solve_for_root(ops: &HashMap<String, Vec<String>>, ans: &mut HashMap<String, i64>) {
    let rop = ops.get("root").unwrap();
    let mut q: VecDeque<&str> = VecDeque::new();
    //
    if let Some(x) = ans.get(&rop[0]) {
        q.push_back(&rop[2]);
        ans.insert(rop[2].to_string(), *x);
    }
    //
    if let Some(x) = ans.get(&rop[2]) {
        q.push_back(&rop[0]);
        ans.insert(rop[0].to_string(), *x);
    }

    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        let &calc = ans.get(cur).unwrap();
        let cop = ops.get(cur).unwrap();
        if cur == "humn" {
            break;
        }
        match (ans.get(&cop[0]), ans.get(&cop[2])) {
            (Some(x), None) => {
                let other_ele = calc1(calc, cop[1].as_ref(), *x);
                ans.insert(cop[2].to_string(), other_ele);
                q.push_back(cop[2].as_str());
            }
            (None, Some(x)) => {
                let other_ele = calc2(calc, cop[1].as_str(), *x);
                ans.insert(cop[0].to_string(), other_ele);
                q.push_back(cop[0].as_str());
            }
            (Some(_), Some(_)) => {}
            _ => {
                todo!();
                // 0
            }
        };
        if ans.get(&cop[0]) == None {
            q.push_back(cop[0].as_ref());
        }
    }
}

// ans = ele op to_be_calced
fn calc1(ans: i64, op: &str, ele: i64) -> i64 {
    // println!("{ans} = {ele} {op} x");
    match op {
        "*" => ans / ele,
        "/" => ele / ans,
        "-" => ele - ans,
        "+" => ans - ele,
        _ => todo!(),
    }
}
// ans = to_be_calced op  ele
fn calc2(ans: i64, op: &str, ele: i64) -> i64 {
    // println!("{ans} = x {op} {ele}");
    match op {
        "*" => ans / ele,
        "/" => ans * ele,
        "-" => ans + ele,
        "+" => ans - ele,
        _ => todo!(),
    }
}
