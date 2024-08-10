fn main() {
    let (a, b) =  include_str!("../input.txt").split_once("\n").unwrap();
    let aa = a.split(" ").filter_map( |s: &str| 
        s.parse::<i64>().ok()
    ).collect::<Vec<i64>>();
    let bb = b.split(" ").filter_map( |s: &str| 
        s.parse::<i64>().ok()
    ).collect::<Vec<i64>>();
    let input = aa.iter().zip(bb.iter()).map(|(&a, &b)| {
        (a,b)
    }).collect::<Vec<(i64,i64)>>();
    let ans = input.iter().map(|&(a,b)| {
        let x = option(b, 0, a/2, a);
        println!("{}",x);
       let ans = (a-x)- x +1;
       ans
    }).product::<i64>();
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