fn main() {
    let s = include_str!("../input.txt");
    let field = s
        .split("\n")
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut ans = 0;
    s.split("\n").enumerate().for_each(|(x, s)| {
        let mut n = "".to_string();
        let mut first = 0;
        let mut last = 0;
        for (y, c) in s.chars().enumerate() {
            let d = c as i32 - '0' as i32;
            if d >= 0 && d < 10 {
                if n == "" {
                    first = y;
                }
                last = y;
                n = format!("{}{}", n, c);
            } else if n != "" {
                let has_symbol = near_symbol(x, first, last, field.len(), field[0].len(), &field);
                if has_symbol {
                    ans += n.parse::<i32>().unwrap();
                }
                n = "".to_string();
            }
        }
        if n != "" {
            let has_symbol = near_symbol(x, first, last, field.len(), field[0].len(), &field);
            if has_symbol {
                ans += n.parse::<i32>().unwrap();
            }
        }
    });
    print!("{}", ans);
}

fn near_symbol(
    x: usize,
    first: usize,
    last: usize,
    maxx: usize,
    maxy: usize,
    field: &Vec<Vec<char>>,
) -> bool {
    for i in vec![x as i32 - 1, x as i32 + 1] {
        if i < 0 || i >= maxx as i32 {
            continue;
        }
        for j in (first as i32 - 1)..=(last as i32 + 1) {
            if j < 0 || j >= maxy as i32 {
                continue;
            }
            let c = field[i as usize][j as usize];
            if is_symbol(c) {
                return true;
            }
        }
    }
    if first as i32 - 1 >= 0 {
        let c = field[x][first - 1];
        if is_symbol(c) {
            return true;
        }
    }
    if last + 1 < maxy {
        let c = field[x][last + 1];
        if is_symbol(c) {
            return true;
        }
    }
    false
}
fn is_symbol(c: char) -> bool {
    let d = c as i32 - '0' as i32;
    if d >= 0 && d < 10 {
        return true;
    }
    c != '.'
}
