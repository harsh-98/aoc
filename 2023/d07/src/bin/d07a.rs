use core::cmp::Ordering;
use std::collections::HashMap;
fn main() {
   let mut entry=include_str!("../input.txt")
   .lines()
   .map(|s| s.split_once(" ").unwrap())
   .map(|e|{
    (e.0, priority(e.0), e.1.parse::<i64>().unwrap())
}).collect::<Vec<(&str, CardSet, i64)>>();
entry.sort_by(|a, b| cmp(a.0 ,&a.1, b.0, &b.1));

println!("{:?}", entry);
let ans = entry.iter().enumerate().map(|(i, e)| e.2*(i as i64+1)).sum::<i64>();
println!("{}", ans);
}

#[derive(PartialEq, PartialOrd, Debug)]
enum CardSet {
    InValid,
    Kind1,
    Kind2,
    TwoPair,
    Kind3,
    FullHouse,
    Kind4,
    Kind5,
}
fn priority(a: &str) -> CardSet {
    let mut m = HashMap::<char, i64>::new();
    a.chars().for_each(|c| 
        {m.entry(c).and_modify(|e| *e += 1).or_insert(1);}
    );
    if m.len() == 1 {
        return CardSet::Kind5;
    } else if m.len() == 2 {
        for (_,&v ) in m.iter() {
            if v == 4 || v == 1 {
                return CardSet::Kind4;
            }
            if v == 3 || v == 2 {
                return CardSet::FullHouse;
            }
        }
        return CardSet::Kind4;
    } else if m.len() == 3 {
        for (k,&v ) in m.iter() {
            if v == 2 {
                return CardSet::TwoPair;
            }
            if v == 3  {
                return CardSet::Kind3;
            }
        }
    } else if m.len() == 4 {
        return CardSet::Kind2;
    } else {
        return CardSet::Kind1;
    }
    return CardSet::InValid;
}
fn cmp(a: &str, ap: &CardSet,  b: &str, bp: &CardSet) -> Ordering {
    let M: HashMap<char, i64> = HashMap::<char, i64>::from([
        ('A', 10),
    ('K', 9),
    ('Q', 8),
    ('J', 7),
    ('T',6),
    ('9', 5),
    ('8', 4),
    ('7', 3),
    ('6', 2),
    ('5', 1),
    ('4', 0),
    ('3', -1),
    ('2', -2),
    ]);
    if ap > bp {
        return Ordering::Greater;
    } else if ap < bp {
        return Ordering::Less;
    } else {
        for (ind, ae) in a.chars().enumerate() {
            let be = b.chars().nth(ind).unwrap();
            if be == ae {
                continue;
            }
            return  M.get(&ae).unwrap().cmp(M.get(&be).unwrap());
        }
        return Ordering::Equal;
    }
}