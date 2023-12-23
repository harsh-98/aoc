fn main() {
    let x = include_str!("../../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(a, b)| match (a, b) {
            ("A", "X") => 3,
            ("B", "X") => 1,
            ("C", "X") => 2,
            ("A", "Y") => 3 + 1,
            ("B", "Y") => 3 + 2,
            ("C", "Y") => 3 + 3,
            ("A", "Z") => 6 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "Z") => 6 + 1,
            (_, _) => 0,
        })
        .sum::<i32>();
    println!("{:?}", x);
}
