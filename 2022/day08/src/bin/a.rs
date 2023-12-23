fn main() {
    let matrix = include_str!("../../input.txt")
        .lines()
        .map(|s| s.chars().map(|e| e as u8 - '0' as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let l = matrix.len();

    let mut d: Vec<Vec<u8>> = vec![vec![0; l]; l];

    (1..l - 1).for_each(|x| {
        let mut check = matrix[x][l - 1];
        (1..(l - 1)).rev().for_each(|y| {
            if matrix[x][y] > check {
                check = matrix[x][y];
                d[x][y] = 1;
            }
        });
        let mut check = matrix[x][0];
        (1..(l - 1)).for_each(|y| {
            if matrix[x][y] > check {
                check = matrix[x][y];
                d[x][y] = 1;
            }
        });
    });

    (1..l - 1).for_each(|y| {
        let mut check = matrix[0][y];
        (1..(l - 1)).for_each(|x| {
            if matrix[x][y] > check {
                check = matrix[x][y];
                d[x][y] = 1;
            }
        });
        let mut check = matrix[l - 1][y];
        (1..(l - 1)).rev().for_each(|x| {
            if matrix[x][y] > check {
                check = matrix[x][y];
                d[x][y] = 1;
            }
        });
    });

    let mut s = d
        .iter()
        .map(|m| m.iter().filter(|&&e| e > 0).count() as i32)
        .sum::<i32>();
    s = matrix.len() as i32 * 4 - 4 + s;
    println!("{}", s);
}
