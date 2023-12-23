use regex::Regex;
fn main() {
    let (m, steps) = include_str!("../../input.txt").split_once("\n\n").unwrap();
    let mut obj: Vec<Vec<char>> = Vec::new();
    m.lines().rev().skip(1).for_each(|s| {
        let c = s.chars().collect::<Vec<char>>();
        let mut ind = 0;
        while ind * 4 + 1 < c.len() {
            if obj.len() < ind + 1 {
                obj.push(Vec::new());
            }
            match c[ind * 4 + 1] {
                ' ' => (),
                d => obj[ind].push(d),
            };
            ind += 1;
        }
    });
    println!("Starting:{:?}", obj);
    let re = Regex::new(r"(\d+)").unwrap();
    steps.lines().for_each(|n| {
        let x = re
            .captures_iter(n)
            .map(|c| c[1].parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let l = obj[(x[1] - 1) as usize].len();
        let removed_eles = obj[(x[1] - 1) as usize]
            .drain((l - x[0] as usize)..)
            .rev()
            .collect::<Vec<char>>();
        obj[(x[2] - 1) as usize].extend(removed_eles);
    });
    println!("Finish:{:?}", obj);
    let msg = obj.iter().fold(String::new(), |total, n| {
        format!("{}{}", total, n.last().unwrap_or_else(|| { &' ' }))
    });
    println!("{}", msg);
}
