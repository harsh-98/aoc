fn main() {
    let vector = day07::solve();
    let total = vector.iter().filter(|&&e| e <= 100_000).sum::<i32>();
    println!("{}", total);
}
