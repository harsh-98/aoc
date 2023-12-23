fn main() {
    println!(
        "{:?}",
        include_str!("../input.txt")
            .split("\n")
            .map(|s| {
                let mut first = -1;
                let mut last = 0;
                for i in s.chars() {
                    let d = i as u32 - '0' as u32;
                    if (i as u32 - '0' as u32) < 10 {
                        if first == -1 {
                            first = d as i32;
                        }
                        last = d as i32;
                    }
                }
                first * 10 + last
            })
            .sum::<i32>()
    );
}
