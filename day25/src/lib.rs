#[allow(non_snake_case)]

pub fn to10FromStr(s: &str) -> i64 {
    let mut mul: i64 = 1;
    let mut ans: i64 = 0;
    for c in s.chars().rev() {
        let d: i64 = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => todo!(),
        };
        ans += d * mul;
        mul *= 5;
    }
    ans
}
#[allow(non_snake_case)]
pub fn to5FromNum(mut num: i64) -> String {
    if num > -2 && num <= 2 {
        return numToChar(num).to_string();
    }
    //
    let mut x = 1;
    let mut count = 0;
    while num / (((5 * x) / 2) + 1) != 0 {
        x *= 5;
        count += 1;
    }
    let mut ans = String::new();
    if num <= (x * 5) / 2 && num > (x * 3) / 2 {
        ans.push('2');
        addZero(num - 2 * x, count, &mut ans);
    } else if num <= (x * 3) / 2 && num > (x * 1) / 2 {
        ans.push('1');
        addZero(num - 1 * x, count, &mut ans);
    } else if num <= (x * 1) / 2 && num > (x * -1) / 2 {
        ans.push('0');
        addZero(num, count, &mut ans);
    } else if num <= (x * -1) / 2 && num > (x * -3) / 2 {
        ans.push('-');
        addZero(num + 1 * x, count, &mut ans);
    } else if num <= (x * -3) / 2 && num > (x * -5) / 2 {
        ans.push('=');
        addZero(num + 2 * x, count, &mut ans);
    } else {
        panic!("");
    }
    ans
}

#[allow(non_snake_case)]
fn addZero(num: i64, mut count: usize, ans: &mut String) {
    let s = to5FromNum(num);
    while count > s.len() {
        ans.push('0');
        count -= 1;
    }
    ans.extend(s.chars());
}
#[allow(non_snake_case)]
fn numToChar(num: i64) -> char {
    match num {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => todo!(),
    }
}
fn neg(c: char) -> char {
    match c {
        '2' => '=',
        '1' => '-',
        '0' => '0',
        '-' => '1',
        '=' => '2',
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // let result = crate::to5FromNum(3);
        // assert_eq!(result, "1=");
        let result = crate::to5FromNum(201);
        assert_eq!(result, "2=01");
        let result = crate::to5FromNum(1747);
        assert_eq!(result, "1=-0-2");
    }
}
