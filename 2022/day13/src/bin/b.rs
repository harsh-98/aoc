use day13::{compare, Cmp};
use std::cmp::Ordering;
fn main() {
    let mut eles = include_str!("../../input.txt")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| to_vec(s))
        .collect::<Vec<Vec<char>>>();

    eles.push(to_vec("[[2]]"));
    eles.push(to_vec("[[6]]"));

    eles.sort_by(|x, y| match compare(&x[..], &y[..]) {
        Cmp::Eq => Ordering::Equal,
        Cmp::Less => Ordering::Greater,
        Cmp::More => Ordering::Less,
    });
    //
    eles.iter()
        .for_each(|s| println!("{}", s.iter().collect::<String>()));

    let ans = eles
        .iter()
        .enumerate()
        .filter_map(|(ind, s)| {
            let s = s.iter().collect::<String>();
            if s == "[[2]]" || s == "[[6]]" {
                return Some(ind + 1);
            }
            None
        })
        .product::<usize>();
    println!("{}", ans);
}

fn to_vec(s: &str) -> Vec<char> {
    s.chars().collect::<Vec<char>>()
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
