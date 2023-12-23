fn main() {
    let digitals: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];
    println!(
        "{:?}",
        include_str!("../input.txt")
            .split("\n")
            .filter_map(|s| {
                let letters = s.chars().collect::<Vec<char>>();
                let l = letters.len();
                let mut first = -1;
                let mut last = 0;
                for i in 0..(l - 2) {
                    let mut number = -1;
                    let d = letters[i] as i32 - '0' as i32;
                    if d >= 0 && d < 10 {
                        number = d;
                    } else {
                        let l3 = s.get(i..(i + 3)).unwrap();
                        if let Some(index) = digitals.iter().position(|&r| r == l3) {
                            number = index as i32 + 1;
                        }
                        {
                            if l > 3 && i < l - 3 {
                                let l4 = s.get(i..(i + 4)).unwrap();
                                if let Some(index) = digitals.iter().position(|&r| r == l4) {
                                    number = index as i32 + 1;
                                }
                            }
                        }
                        {
                            if l > 4 && i < l - 4 {
                                let l5 = s.get(i..(i + 5)).unwrap();
                                if let Some(index) = digitals.iter().position(|&r| r == l5) {
                                    number = index as i32 + 1;
                                }
                            }
                        }
                    }
                    if number >= 0 {
                        if first == -1 {
                            first = number as i32;
                        }
                        last = number as i32;
                    }
                }
                for sub in 1..=2 {
                    let number = letters[l + sub - 3] as i32 - '0' as i32;
                    if number >= 0 && number < 10 {
                        if first == -1 {
                            first = number;
                        }
                        last = number;
                    }
                }

                if first == -1 {
                    return None;
                } else {
                    Some(first * 10 + last)
                }
            })
            .sum::<i32>()
    );
}
