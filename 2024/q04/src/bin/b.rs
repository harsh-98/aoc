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
            //
            let mut tt = false;
            for t in "MS".chars() {
                tt = tt
                    || check(&x, i + 1, j + 1) == t
                        && (check(&x, i - 1, j + 1) == t || check(&x, i + 1, j - 1) == t);
            }
            //
            let mut ff = false;
            for t in "MS".chars() {
                ff = ff
                    || check(&x, i - 1, j - 1) == t
                        && (check(&x, i - 1, j + 1) == t || check(&x, i + 1, j - 1) == t);
            }

            if check(&x, i, j) == 'A'
                && tt
                && ff
                && check(&x, i - 1, j - 1) != check(&x, i + 1, j + 1)
            {
                total = total + 1;
            }
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
