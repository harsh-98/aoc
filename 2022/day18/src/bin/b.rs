fn main() {
    let (v, total) = day18::get_cubes();

    let maxi = v.iter().fold((0, 0, 0), |a, b| {
        (i32::max(a.0, b.0), i32::max(a.1, b.1), i32::max(a.2, b.2))
    });

    let mut matrix = vec![
        vec![vec![false; (maxi.2 + 1) as usize]; (maxi.1 + 1) as usize];
        (maxi.0 + 1) as usize
    ];

    v.iter().for_each(|a| {
        matrix[a.0 as usize][a.1 as usize][a.2 as usize] = true;
    });
    let mut trapped = Vec::new();
    for i in 0..=maxi.0 {
        for j in 0..=maxi.1 {
            for k in 0..=maxi.2 {
                if !matrix[i as usize][j as usize][k as usize] {
                    let x = bfs(&mut matrix, (i, j, k), &maxi);
                    trapped.extend(x);
                }
            }
        }
    }
    let mut inward = 0;
    for a in v.iter() {
        for b in trapped.iter() {
            if day18::cmp(
                &(a.0 as i32, a.1 as i32, a.2 as i32),
                &(b.0 as i32, b.1 as i32, b.2 as i32),
            ) {
                inward += 1;
            }
        }
    }
    let outer = total - inward;
    println!("{outer}");
}

fn bfs(
    matrix: &mut Vec<Vec<Vec<bool>>>,
    p: (i32, i32, i32),
    maxi: &(i32, i32, i32),
) -> Vec<(i32, i32, i32)> {
    let dirs = [
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, -1, 0),
        (1, 0, 0),
        (-1, 0, 0),
    ];
    let mut q = Vec::new();
    q.push(p);
    matrix[p.0 as usize][p.1 as usize][p.2 as usize] = true;
    //
    let mut pop_ind = 0;
    let mut open = false;
    while pop_ind < q.len() {
        let cur = q[pop_ind].clone();
        pop_ind += 1;
        if cur.0 == 0
            || cur.1 == 0
            || cur.2 == 0
            || cur.0 == maxi.0
            || cur.1 == maxi.1
            || cur.2 == maxi.2
        {
            open = true;
        }

        for dir in dirs.iter() {
            if let Some(new_cord) = check_in_boundary(&cur, dir, maxi) {
                if !matrix[new_cord.0 as usize][new_cord.1 as usize][new_cord.2 as usize] {
                    matrix[new_cord.0 as usize][new_cord.1 as usize][new_cord.2 as usize] = true;
                    q.push(new_cord);
                }
            }
        }
    }
    if open {
        Vec::new()
    } else {
        q
    }
}
fn check_in_boundary(
    cur: &(i32, i32, i32),
    dir: &(i32, i32, i32),
    maxi: &(i32, i32, i32),
) -> Option<(i32, i32, i32)> {
    let x = cur.0 + dir.0;
    let y = cur.1 + dir.1;
    let z = cur.2 + dir.2;
    if x >= 0 && x <= maxi.0 && y >= 0 && y <= maxi.1 && z >= 0 && z <= maxi.2 {
        Some((x, y, z))
    } else {
        None
    }
}
