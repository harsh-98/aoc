fn main() {
    let matrix = include_str!("../../input.txt")
        .lines()
        .map(|s| s.chars().map(|e| e as u8 - '0' as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let l = matrix.len();
    let mut max_ans = 0;
    for x in 0..l {
        for y in 0..l {
            let a = max_view(&matrix[x][y..]);
            let v = matrix[x][0..=y]
                .iter()
                .map(|&e| e)
                .rev()
                .collect::<Vec<u8>>();
            let b = max_view(&v);

            let j = matrix.iter().map(|e| e[y]).collect::<Vec<u8>>();

            let c = max_view(&j[x..]);
            let v = j[0..=x].iter().map(|&e| e).rev().collect::<Vec<u8>>();
            let d = max_view(&v);
            let t = a * b * c * d;
            // println!("{x} {y} {} {} {} {}: {}", a, b, c, d, t);
            max_ans = usize::max(max_ans, t);
        }
    }
    println!("{}", max_ans);
}

fn max_view(v: &[u8]) -> usize {
    if v.len() == 1 {
        return 0;
    }
    let check = v[0];
    let mut count = 0;
    for &tt in v.iter().skip(1) {
        count += 1;
        if tt >= check {
            break;
        }
    }
    return count;
}
