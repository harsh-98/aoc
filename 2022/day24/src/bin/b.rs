fn main() {
    let (x_max, y_max, set) = day24::solve();
    let mut time = day24::time_it(x_max, y_max, &set, (-1, 0, 0), (x_max, y_max - 1));
    println!("{time}");
    time = day24::time_it(x_max, y_max, &set, (x_max, y_max - 1, time), (-1, 0));
    println!("{time}");
    time = day24::time_it(x_max, y_max, &set, (-1, 0, time), (x_max, y_max - 1));
    println!("{time}");
}
