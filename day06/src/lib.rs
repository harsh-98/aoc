use std::collections::HashMap;
pub fn solve(num: usize) -> usize {
    let ans = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let s = s.chars().collect::<Vec<char>>();
            let mut x: HashMap<char, usize> = HashMap::new();
            for i in 0..(num - 1) {
                x.insert(s[i], x.get(&s[i]).unwrap_or(&0) + 1);
            }
            for i in (num - 1)..s.len() {
                if i as i32 - num as i32 >= 0 {
                    x.insert(s[i - num], x[&s[i - num]] - 1);
                }
                let mut detected = x.get(&s[i]).unwrap_or(&0) == &0;
                for checkI in 1..(num - 1) {
                    detected = detected && x.get(&s[i - checkI]).unwrap_or(&0) <= &1;
                }
                if detected {
                    return i + 1;
                }
                x.insert(s[i], x.get(&s[i]).unwrap_or(&0) + 1);
            }
            return 0;
        })
        .sum::<usize>();
    ans
}
