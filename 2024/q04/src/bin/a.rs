fn main() {
    let x = include_str!("../../input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let word = "XMAS".chars().collect::<Vec<char>>();
    let mut total = 0;
    for i in 0..(x.len() as i32) {
        for j in 0..(x[0].len() as i32) {
            let mut ans = vec![true; 8];
            for k in 0..4 {
                ans[0] = ans[0] & (check(&x, i, j + k) == *word.get(k as usize).unwrap());
                ans[1] = ans[1] & (check(&x, i + k, j) == *word.get(k as usize).unwrap());
                ans[2] = ans[2] & (check(&x, i, j - k) == *word.get(k as usize).unwrap());
                ans[3] = ans[3] & (check(&x, i - k, j) == *word.get(k as usize).unwrap());
                ans[4] = ans[4] & (check(&x, i - k, j - k) == *word.get(k as usize).unwrap());
                ans[5] = ans[5] & (check(&x, i + k, j + k) == *word.get(k as usize).unwrap());
                ans[6] = ans[6] & (check(&x, i - k, j + k) == *word.get(k as usize).unwrap());
                ans[7] = ans[7] & (check(&x, i + k, j - k) == *word.get(k as usize).unwrap());
            }
            total = total + ans.iter().fold(0, |y, &x| if x { y + 1 } else { y });
        }
    }
    println!("{:?}", total);
}

fn check(v: &Vec<Vec<char>>, i: i32, j: i32) -> char {
    if i < 0 {
        return '0';
    } else if j < 0 {
        return '0';
    } else if j >= v[0].len() as i32 {
        return '0';
    } else if i >= v.len() as i32 {
        return '0';
    }
    let x = v.get(i as usize);
    if let Some(t) = x {
        return *t.get(j as usize).unwrap_or_else(|| &'0');
    }
    '0'
}
