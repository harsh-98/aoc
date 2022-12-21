fn main() {
    let (dependent, ops, mut ans) = day21::get_data();

    for (k, v) in ops.iter() {
        if v.len() == 1 {
            day21::calc_eles(k, &ops, &dependent, &mut ans);
        }
    }
    println!("{:?}", ans);
    println!("{}", ans.get("root").unwrap())
}
