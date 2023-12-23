use std::collections::{HashMap, VecDeque};

pub fn calc_eles(
    start: &str,
    ops: &HashMap<String, Vec<String>>,
    dependent: &HashMap<String, Vec<String>>,
    ans: &mut HashMap<String, i64>,
) {
    let mut q = VecDeque::new();
    q.push_back(start);
    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        if let Some(arr) = dependent.get(cur) {
            for ele in arr {
                let op = ops.get(ele).unwrap();
                match (ans.get(&op[0]), ans.get(&op[2])) {
                    (Some(v1), Some(v2)) => {
                        let calc = match op[1].as_ref() {
                            "*" => v1 * v2,
                            "/" => v1 / v2,
                            "+" => v1 + v2,
                            "-" => v1 - v2,
                            _ => 0,
                        };
                        ans.insert(ele.to_string(), calc);
                        q.push_back(ele);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn get_data() -> (
    HashMap<String, Vec<String>>,
    HashMap<String, Vec<String>>,
    HashMap<String, i64>,
) {
    let mut dependent: HashMap<String, Vec<String>> = HashMap::new();
    let mut ops = HashMap::new();
    let mut ans = HashMap::new();
    include_str!("../input.txt").lines().for_each(|x| {
        let (root, num) = x.split_once(": ").unwrap();
        let op = num
            .split(" ")
            .map(|y| y.to_string())
            .collect::<Vec<String>>();
        if op.len() == 1 {
            ans.insert(root.to_string(), op[0].parse::<i64>().unwrap());
        } else {
            if dependent.contains_key(&op[0]) {
                dependent.get_mut(&op[0]).unwrap().push(root.to_string());
            } else {
                dependent.insert(op[0].to_string(), vec![root.to_string()]);
            }
            if dependent.contains_key(&op[2]) {
                dependent.get_mut(&op[2]).unwrap().push(root.to_string());
            } else {
                dependent.insert(op[2].to_string(), vec![root.to_string()]);
            }
        }
        ops.insert(root.to_string(), op);
    });
    (dependent, ops, ans)
}
