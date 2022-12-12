fn main() {
    let x = include_str!("../../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(a, b)| match (a, b) {
            ("A", "X") => 3 + 1,
            ("B", "Y") => 3 + 2,
            ("C", "Z") => 3 + 3,
            ("A", "Y") => 6 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "X") => 6 + 1,
            ("A", "Z") => 3,
            ("B", "X") => 1,
            ("C", "Y") => 2,
            (_, _) => 0,
        })
        .sum::<i32>();
    println!("{:?}", x);
}
