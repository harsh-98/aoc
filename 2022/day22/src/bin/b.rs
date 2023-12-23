use day22::{solve, Dir};
// let sq = vec![(0, 8), (8, 12), (8, 8), (4, 4), (4, 8), (4, 0)];

const SQ: [(usize, usize); 6] = [(0, 50), (0, 100), (100, 50), (100, 0), (50, 50), (150, 0)];
const SQSIZE: usize = 50;
const _49: usize = SQSIZE - 1;
fn new_cord_dir(
    matrix: &Vec<Vec<char>>,
    dir: Dir,
    start: &(usize, usize),
) -> ((usize, usize), Dir) {
    let sqInd = SQ
        .iter()
        .enumerate()
        .filter_map(|(ind, bod)| {
            if bod.0 + SQSIZE > start.0
                && start.0 >= bod.0
                && bod.1 + SQSIZE > start.1
                && start.1 >= bod.1
            {
                return Some(ind);
            }
            None
        })
        .last()
        .unwrap();

    let inSqCord = (start.0 - SQ[sqInd].0, start.1 - SQ[sqInd].1);
    // panic!("{sqInd} {inSqCord:?}");
    //  01
    //  4
    // 32
    // 5
    match sqInd {
        0 => match dir {
            Dir::Up => {
                let cord = (SQ[5].0 + inSqCord.1, SQ[5].1);
                return (cord, Dir::Right);
            }
            Dir::Left => {
                let cord = (SQ[3].0 + _49 - inSqCord.0, SQ[3].1);
                return (cord, Dir::Right);
            }
            _ => todo!(),
        },
        1 => match dir {
            Dir::Up => {
                let cord = (SQ[5].0 + _49, SQ[5].1 + inSqCord.1);
                return (cord, Dir::Up);
            }
            Dir::Down => {
                let cord = (SQ[4].0 + inSqCord.1, SQ[4].1 + _49);
                return (cord, Dir::Left);
            }
            Dir::Right => {
                let cord = (SQ[2].0 + _49 - inSqCord.0, SQ[2].1 + _49);
                return (cord, Dir::Left);
            }
            _ => todo!(),
        },
        2 => match dir {
            Dir::Right => {
                let cord = (SQ[1].0 + _49 - inSqCord.0, SQ[1].1 + _49);
                return (cord, Dir::Left);
            }
            Dir::Down => {
                let cord = (SQ[5].0 + inSqCord.1, SQ[5].1 + _49);
                return (cord, Dir::Left);
            }
            _ => todo!(),
        },
        3 => match dir {
            Dir::Left => {
                let cord = (SQ[0].0 + _49 - inSqCord.0, SQ[0].1);
                return (cord, Dir::Right);
            }
            Dir::Up => {
                let cord = (SQ[4].0 + inSqCord.1, SQ[4].1);
                return (cord, Dir::Right);
            }
            _ => todo!(),
        },
        4 => match dir {
            Dir::Left => {
                let cord = (SQ[3].0, SQ[3].1 + inSqCord.0);
                return (cord, Dir::Down);
            }
            Dir::Right => {
                let cord = (SQ[1].0 + _49, SQ[1].1 + inSqCord.0);
                return (cord, Dir::Up);
            }
            _ => todo!(),
        },
        5 => match dir {
            Dir::Left => {
                let cord = (SQ[0].0, SQ[0].1 + inSqCord.0);
                return (cord, Dir::Down);
            }
            Dir::Down => {
                let cord = (SQ[1].0, SQ[1].1 + inSqCord.1);
                return (cord, Dir::Down);
            }
            Dir::Right => {
                let cord = (SQ[2].0 + _49, SQ[2].1 + inSqCord.0);
                return (cord, Dir::Up);
            }
            _ => todo!(),
        },
        _ => todo!(),
    };
    todo!()
}

fn main() {
    solve(Box::new(new_cord_dir))
}
