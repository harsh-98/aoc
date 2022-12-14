#![feature(array_windows)]
pub fn solve(is_floor_present: bool) -> usize {
    let mut y_max = 0;
    let mut cord_lines = include_str!("../input.txt")
        .lines()
        // .array_chunks()
        .map(|a| {
            let (a, _max_y) = split_to_cord(a);
            y_max = usize::max(y_max, _max_y);
            a
        })
        .collect::<Vec<Vec<(usize, usize)>>>();

    y_max += 3;
    let last = 500 - y_max;

    //
    for arr in cord_lines.iter_mut() {
        arr.iter_mut().for_each(|(x, _)| *x = *x - last);
    }

    let x_max = y_max * 2;
    println!("x_len:{} y_len:{}", x_max, y_max);
    //
    let mut matrix = vec![vec!['.'; y_max]; x_max];
    if is_floor_present {
        matrix.iter_mut().for_each(|x| x[y_max - 1] = '#');
    }
    //
    let mut add_rocks = |p: &[(usize, usize); 2]| {
        let p2 = p[0];
        let p1 = p[1];
        if p1.0 == p2.0 {
            for other_cord in usize::min(p1.1, p2.1)..usize::max(p1.1, p2.1) + 1 {
                matrix[p1.0][other_cord] = '#';
            }
        }
        if p1.1 == p2.1 {
            for other_cord in usize::min(p1.0, p2.0)..usize::max(p1.0, p2.0) + 1 {
                matrix[other_cord][p1.1] = '#';
            }
        }
    };

    for arr in cord_lines.iter() {
        arr.array_windows().for_each(&mut add_rocks);
    }

    //
    let start = (500 - last, 0);
    let mut count = 0;
    while matrix[start.0][start.1] != 'o' && fall(&mut matrix, start) {
        count += 1;
    }
    print_m(&matrix);
    count
}

fn print_m(matrix: &Vec<Vec<char>>) {
    matrix
        .iter()
        .for_each(|l| println!("{}", l.iter().collect::<String>()));
}

fn fall(matrix: &mut Vec<Vec<char>>, mut start: (usize, usize)) -> bool {
    loop {
        if start.0 == matrix.len() - 1 || start.1 == matrix[0].len() - 1 {
            return false;
        }
        if !filled(matrix[start.0][start.1 + 1]) {
            start.1 += 1;
        } else if !filled(matrix[start.0 - 1][start.1 + 1]) {
            start = (start.0 - 1, start.1 + 1);
        } else if !filled(matrix[start.0 + 1][start.1 + 1]) {
            start = (start.0 + 1, start.1 + 1);
        } else {
            matrix[start.0][start.1] = 'o';
            return true;
        }
    }
}

fn filled(x: char) -> bool {
    x == '#' || x == 'o'
}

fn split_to_cord(a: &str) -> (Vec<(usize, usize)>, usize) {
    let mut max_y = 0;
    let v = a
        .split(" -> ")
        .map(|p| p.split_once(",").unwrap())
        .map(|(x, y)| {
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            max_y = usize::max(y, max_y);
            (x, y)
        })
        .collect::<Vec<(usize, usize)>>();
    (v, max_y)
}
