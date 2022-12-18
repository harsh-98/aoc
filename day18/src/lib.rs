pub fn cmp(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> bool {
    if a.0 == b.0 && a.1 == b.1 {
        return i32::abs(b.2 - a.2) == 1;
    } else if a.0 == b.0 && a.2 == b.2 {
        return i32::abs(b.1 - a.1) == 1;
    } else if a.1 == b.1 && a.2 == b.2 {
        return i32::abs(b.0 - a.0) == 1;
    }
    false
}

pub fn get_cubes() -> (Vec<(i32, i32, i32)>, usize) {
    let v = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let v = s
                .split(",")
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (v[0], v[1], v[2])
        })
        .collect::<Vec<(i32, i32, i32)>>();

    let mut sub = 0;
    for i in 0..(v.len() - 1) {
        for j in (i + 1)..v.len() {
            if cmp(&v[i], &v[j]) {
                sub += 1;
            }
        }
    }
    let total = v.len() * 6 - 2 * sub;
    (v, total)
}
