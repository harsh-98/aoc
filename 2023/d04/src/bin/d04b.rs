use std::collections::HashMap;
fn main() {
    let mut m: HashMap<usize, i32> = HashMap::new();
    include_str!("../input.txt")
        .split("\n")
        .enumerate()
        .for_each(|(i, _)| {
            m.insert(i, 1);
        });
    //
    include_str!("../input.txt")
        .split("\n")
        .enumerate()
        .for_each(|(ind, s)| {
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
                    ans += 1;
                }
            }
            for i in 1..=ans {
                let _ind = ind + i;
                let prev = m.get(&_ind).unwrap();
                m.insert(_ind, prev + m[&ind]);
            }
        });
    let result = m.values().sum::<i32>();
    println!("{}", result);
}
