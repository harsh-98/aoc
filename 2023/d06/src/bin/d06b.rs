fn main() {
    let (a, b) =  include_str!("../input.txt").split_once("\n").unwrap();
    let aa = a.split_once(" ").unwrap().1.chars().filter_map( |s: char| 
        if s == ' ' {
            None
        } else {
            Some(s)
        }
    ).collect::<String>().parse::<i64>().unwrap();
    let bb = b.split_once(" ").unwrap().1.chars().filter_map( |s: char| 
        if s == ' ' {
            None
        } else {
            Some(s)
        }
    ).collect::<String>().parse::<i64>().unwrap();
        let x = option(bb, 0, aa/2, aa);
        println!("{}",x);
       let ans = (aa-x)- x +1;
    println!("{}", ans);
}

fn option(max: i64, mut x: i64, mut y: i64, t: i64) -> (i64) {
    while x<= y {
        let mid = (x+y)/2;
        let a = (t-mid) *mid;
        if max > a{
            x= mid+1;
        } else if  max < a {
            y = mid-1;
        } else {
            return x+1;
        }
    }
    return x
}