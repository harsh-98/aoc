fn main() {
    let (a, b) = include_str!("../input.txt").split_once("\n\n").unwrap();
   let  seeds = a.split(" ").filter_map(|s: &str| 
        if let Ok(n) =    s.parse::<i64>() {
            Some(n)
        } else {
            None
        }
    ).collect::<Vec<i64>>();
    // println!("{:?}", seeds);

    let dicts = b.split("\n\n").map(|map_set| {
        map_set.lines().skip(1).map(|line|  {
            let eles = line.split(" ").filter_map(|e| {
                match e.parse::<i64>() {
                     Ok(n) => Some(n),
                     Err(e) => {
                        println!("error {}", e);
                        None
                    }
                }
            }).collect::<Vec<i64>>();
            (eles[1], eles[1]+ eles[2]-1,  eles[0])
        }
        ).collect::<Vec<(i64, i64, i64)>>()
        
    }).collect::<Vec<Vec<(i64, i64, i64)>>>();
    let ans = seeds.iter().map(|&_e|  {
        let mut e = _e;
            for m in dicts.iter() {
                for &(l, r, v) in m.iter() {
                    if l <= e && e <= r {
                        e = v+ e-l;
                        break;
                    }
                } 
            }
        e
    }
    ).min();
    println!("{}", ans.unwrap());
}