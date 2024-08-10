fn main() {
    let (a, b) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let mut seeds = Vec::new();
   let  interval = a.split(" ").filter_map(|s: &str| 
        if let Ok(n) =    s.parse::<i64>() {
            Some(n)
        } else {
            None
        }
    ).collect::<Vec<i64>>().chunks(2).map(|e| {
        seeds.push(e[0]);
        seeds.push(e[1]+e[0]-1);
        (e[0], e[1]+e[0]-1)
}).collect::<Vec<(i64,i64)>>();
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
            (eles[1], eles[1]+ eles[2]-1,  eles[0], eles[0]+ eles[2]-1)
        }
        ).collect::<Vec<(i64, i64, i64, i64)>>()
        
    }).collect::<Vec<Vec<(i64, i64, i64, i64)>>>();
    for m in dicts.iter() {
        for &(l, r, v, k) in m.iter() {
            for &(i, j) in interval.iter() {
                if l >= i && l <= j {
                    seeds.push(l);
                }
                if r >= i && r <= j {
                    seeds.push(r);
                }
                if k >= i && k <= j {
                    seeds.push(k);
                }
                if v >= i && v <= j {
                    seeds.push(v);
                }
            }
        }
    }
    let ans = seeds.iter().map(|&_e|  {
        let mut e = _e;
            for m in dicts.iter() {
                for &(l, r, v, _) in m.iter() {
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