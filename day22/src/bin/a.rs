use day22::{solve, Dir};

fn new_cord(matrix: &Vec<Vec<char>>, dir: Dir, start: &(usize, usize)) -> ((usize, usize), Dir) {
    let cord = match dir {
        Dir::Up => {
            let x = matrix[start.0..].partition_point(|x| {
                let xx = if x.len() > start.1 {
                    x[start.1] == '#' || x[start.1] == '.'
                } else {
                    false
                };
                xx
            });
            (x - 1 + start.0, start.1)
        }
        Dir::Down => {
            let x = matrix[..start.0].partition_point(|x| {
                if x.len() > start.1 {
                    x[start.1] == ' '
                } else {
                    true
                }
            });
            (x, start.1)
        }

        Dir::Right => {
            let y = matrix[start.0][..start.1].partition_point(|&x| x == ' ');
            (start.0, y)
        }
        Dir::Left => {
            let y = matrix[start.0][start.1..].partition_point(|&x| x == '#' || x == '.');
            (start.0, y - 1 + start.1)
        }
    };
    return (cord, dir);
}

fn main() {
    solve(Box::new(new_cord))
}
