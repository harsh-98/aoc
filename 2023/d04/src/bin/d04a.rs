use std::collections::HashMap;
fn main() {
    let result = include_str!("../input.txt")
        .split("\n")
        .map(|s| {
            let (_, x) = s.split_once(":").unwrap();
            let (needed, have) = x.split_once("|").unwrap();
            let n = needed
                .trim()
                .split(" ")
                .filter_map(|x| if x != "" { Some(x) } else { None })
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let h = have
                .trim()
                .split(" ")
                .filter_map(|x| if x != "" { Some(x) } else { None })
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let mut nm: HashMap<i32, bool> = HashMap::new();
            for &i in n.iter() {
                nm.insert(i, true);
            }
            let mut ans = 0;
            for i in h.iter() {
                if nm.get(i).is_some() {
                    ans = ans << 1;
                    if ans == 0 {
                        ans = 1;
                    }
                }
            }
            ans
        })
        .sum::<i32>();
    println!("{}", result);
}
