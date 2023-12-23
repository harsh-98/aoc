fn main() {
    let mut vector = Vec::new();
    //
    let mut line = String::new();
    let mut feed = new_line(0);
    //
    let mut value = 0;
    let mut count = 0;
    include_str!("../../input.txt")
        .lines()
        .for_each(|s| match s {
            "noop" => {
                line = format!("{}{}", line, feed[(count % 40) as usize]);
                count += 1;
                if line.len() == 40 {
                    vector.push(line.clone());
                    line = String::new();
                }
            }
            x => {
                count += 2;
                for i in (count - 2)..count {
                    line.push(feed[i % 40]);
                    if line.len() == 40 {
                        vector.push(line.clone());
                        line = String::new();
                    }
                }
                let inc = x.split_once(" ").unwrap().1.parse::<i32>().unwrap();
                value += inc;
                feed = new_line(value);
            }
        });
    //
    vector.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn new_line(n: i32) -> Vec<char> {
    println!("{}", n);
    let mut l = vec!['.'; 40];
    for i in 0..3 {
        if n + i >= 0 && n + i <= 39 {
            l[(n + i) as usize] = '#';
        }
    }
    return l;
}
