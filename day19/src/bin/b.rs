fn main() {
    let ans = day19::solve(32, 3).iter().map(|v| *v).product::<i64>();
    println!("{ans}");
}
