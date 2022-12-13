use std::collections::VecDeque;

fn main() {
    let mut matrix = include_str!("../../input.txt")
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut cord = (0, 0);
    let mut e_cord = (0, 0);
    let x_l = matrix.len();
    let y_l = matrix[0].len();
    for x in 0..x_l {
        for y in 0..y_l {
            if matrix[x][y] == 'S' {
                matrix[x][y] = 'a';
                cord = (x as i32, y as i32);
            }
            if matrix[x][y] == 'E' {
                matrix[x][y] = 'z';
                e_cord = (x as i32, y as i32);
                // break;
            }
        }
    }
    matrix
        .iter()
        .for_each(|s| (println!("{}", s.iter().collect::<String>())));
    let mut score = vec![vec![i32::MAX - 1; matrix[0].len()]; matrix.len()];

    let mut q = VecDeque::new();
    score[cord.0 as usize][cord.1 as usize] = 0;
    q.push_back(cord);

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    while !q.is_empty() {
        let cord = q.pop_front().unwrap();
        for dir in dirs {
            let new_cord = (cord.0 + dir.0, cord.1 + dir.1);
            if in_limit(new_cord, x_l as i32, y_l as i32) {
                let new_cord = (new_cord.0 as usize, new_cord.1 as usize);
                let cord = (cord.0 as usize, cord.1 as usize);
                if matrix[new_cord.0][new_cord.1] as u8 <= (matrix[cord.0][cord.1] as u8 + 1)
                    && score[new_cord.0][new_cord.1] > score[cord.0][cord.1] + 1
                {
                    score[new_cord.0][new_cord.1] = score[cord.0][cord.1] + 1;
                    q.push_back((new_cord.0 as i32, new_cord.1 as i32));
                }
            }
        }
    }
    println!("{}", score[e_cord.0 as usize][e_cord.1 as usize]);
}

fn in_limit(cord: (i32, i32), x: i32, y: i32) -> bool {
    let x = cord.0 >= 0 && cord.0 < x && cord.1 < y && cord.1 >= 0;
    x
}
