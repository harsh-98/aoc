fn main() {
    println!(
        "{:?}",
        include_str!("../input.txt").lines().map(|s| s.chars().filter_map(|c: char| {
            let a =c as i32 - '0' as i32;
            if a < 10 && a >= 0 {
                Some(a)
            } else {
                None
            }
        }).collect::<Vec<i32>>()).map(|s: Vec<i32> | s.first().unwrap()*10+ s.last().unwrap() ).sum::<i32>()
    );
}
