#[derive(PartialEq, Eq)]
pub enum Cmp {
    Less,
    Eq,
    More,
}

pub fn compare(x: &[char], y: &[char]) -> Cmp {
    if x[0] != '[' && y[0] != '[' {
        let x = x.iter().collect::<String>().parse::<i32>().unwrap();
        let y = y.iter().collect::<String>().parse::<i32>().unwrap();
        if x == y {
            return Cmp::Eq;
        } else if x < y {
            return Cmp::More;
        }
        return Cmp::Less;
    } else if x[0] != '[' && y[0] == '[' {
        let new_s = add_bracket(x);
        return compare(&new_s.chars().collect::<Vec<char>>()[..], y);
    } else if x[0] == '[' && y[0] != '[' {
        let new_s = add_bracket(y);
        return compare(x, &new_s.chars().collect::<Vec<char>>()[..]);
    } else {
        let mut x_c = 1;
        let mut y_c = 1;
        loop {
            if x[x_c] == ',' {
                x_c += 1;
            }
            if y[y_c] == ',' {
                y_c += 1;
            }
            //
            let x_ele = get_ele_len(&x[x_c..]);
            let y_ele = get_ele_len(&y[y_c..]);
            //
            //
            if x_ele == 0 {
                if y_ele == 0 {
                    return Cmp::Eq;
                }
                return Cmp::More;
            }
            if y_ele == 0 {
                return Cmp::Less;
            }
            let ans_ele = compare(&x[x_c..(x_c + x_ele)], &y[y_c..(y_c + y_ele)]);
            if ans_ele == Cmp::Eq {
                x_c += x_ele;
                y_c += y_ele;
            } else {
                return ans_ele;
            }
        }
    }
}

fn add_bracket(x: &[char]) -> String {
    let mut new_s = String::new();
    new_s.push('[');
    for &i in x {
        new_s.push(i);
    }
    new_s.push(']');
    return new_s;
}

fn get_ele_len(x: &[char]) -> usize {
    let mut count = 0;
    let mut ans = 0;
    for (ind, ele) in x.iter().enumerate() {
        match ele {
            '[' => {
                count = count + 1;
            }
            ']' => {
                count = count - 1;
                if count < 0 {
                    break;
                }
            }
            ',' => {
                if count == 0 {
                    break;
                }
            }
            _ => {}
        };
        ans = ind + 1; // end
    }
    return ans;
}
