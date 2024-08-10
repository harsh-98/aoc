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

// println!("{:?}", entry);
let ans = entry.iter().enumerate().map(|(i, e)| e.2*(i as i64+1)).sum::<i64>();
println!("{}", ans);
}

#[derive(PartialEq, PartialOrd, Debug)]
enum CardSet {
    InValid,
    Kind1,
    Kind2J,
    Kind2,
    TwoPair,
    Kind3J,
    Kind3,
    FullHouseJ,
    FullHouse,
    Kind4J,
    Kind4,
    Kind5J,
    Kind5,
}
fn priority(a: &str) -> CardSet {
    let mut m = HashMap::<char, i64>::new();
    a.chars().for_each(|c| 
        {m.entry(c).and_modify(|e| *e += 1).or_insert(1);}
    );
    let jcount = m.remove(&'J').unwrap_or_default();
    let mut charc =0;
    let mut maxchar = '2';
    for (&k, &c) in m.iter() {
        if charc <= c {
            charc = c;
            maxchar = k;
        }
    }
    //
    m.entry(maxchar).and_modify(|e| *e += jcount).or_insert(jcount);
    //
    if m.len() == 1 {
        if jcount > 0 {
            // return CardSet::Kind5J;
        }
        return CardSet::Kind5;
    } else if m.len() == 2 {
        for (_,&v ) in m.iter() {
            if v == 4 || v == 1 {
                if jcount > 0 {
                    // return CardSet::Kind4J;
                }
                return CardSet::Kind4;
            }
            if v == 3 || v == 2 {
                if jcount > 0 {
                    // return CardSet::FullHouseJ;
                }
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
                if jcount > 0 {
                    // return CardSet::Kind3J;
                }
                return CardSet::Kind3;
            }
        }
    } else if m.len() == 4 {
        if jcount > 0 {
            // return CardSet::Kind2J;
        }
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
    ('T',6),
    ('9', 5),
    ('8', 4),
    ('7', 3),
    ('6', 2),
    ('5', 1),
    ('4', 0),
    ('3', -1),
    ('2', -2),
    ('J', -3),
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