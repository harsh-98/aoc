fn main() {
    let vector = day07::solve();
    let space_to_delete = vector.last().unwrap() - 4_00_00_000;
    let mut group_space = i32::MAX;
    for i in vector {
        if i >= space_to_delete {
            group_space = i32::min(i, group_space);
        }
    }
    println!("{}", group_space);
}
