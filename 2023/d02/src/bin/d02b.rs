fn main() {
    let r = include_str!("../input.txt")
        .split("\n")
        .map(|s| s.split(": ").collect::<Vec<&str>>()[1])
        .map(|s| {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            s.split("; ")
                .map(|set| set.split(", ").collect::<Vec<&str>>())
                .flatten()
                .map(|d| {
                    let v = d.split(" ").collect::<Vec<&str>>();
                    (v[1], v[0].trim_end_matches(";").parse::<i32>().unwrap())
                })
                .for_each(|(k, v)| match k {
                    "red" => r = i32::max(r, v),
                    "green" => g = i32::max(g, v),
                    "blue" => b = i32::max(b, v),
                    _ => (),
                });
            r * g * b
        })
        .sum::<i32>();
    println!("{:?}", r);
}
