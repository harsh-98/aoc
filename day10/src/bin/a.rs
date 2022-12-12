fn main() {
    let mut vector = Vec::new();
    let mut value: i32 = 1;
    let mut curLim = 20;
    //
    let mut count = 0;
    include_str!("../../input.txt")
        .lines()
        .for_each(|s| match s {
            "noop" => {
                count += 1;
                if curLim <= count && curLim <= 220 {
                    vector.push(value);
                    curLim += 40;
                }
            }
            x => {
                count += 2;
                if curLim <= count && curLim <= 220 {
                    vector.push(value);
                    curLim += 40;
                }
                let inc = x.split_once(" ").unwrap().1.parse::<i32>().unwrap();
                value += inc;
            }
        });
    //
    let mut total = 0;
    for (n, i) in vector.iter().enumerate() {
        println!("{} {}", i, n * 40 + 20);
        total += i * (n * 40 + 20) as i32;
    }
    println!("{}", total);
}
