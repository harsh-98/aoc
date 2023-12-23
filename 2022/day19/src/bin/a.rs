fn main() {
    let ans = day19::solve(24, 30)
        .iter()
        .enumerate()
        .map(|(ind, v)| *v as usize * (ind + 1))
        .sum::<usize>();
    println!("{ans}");
}
