use std::collections::HashMap;
pub fn solve(no_of_rocks: usize) {
    let shapes = vec![
        Shape {
            left: vec![(0, 0)],
            right: vec![(0, 3)],
            extra: vec![(0, 1), (0, 2)],
            bottom: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            cord: None,
        },
        Shape {
            left: vec![(0, 1), (1, 0), (2, 1)],
            right: vec![(0, 1), (1, 2), (2, 1)],
            extra: vec![(1, 1)],
            bottom: vec![(1, 0), (0, 1), (1, 2)],
            cord: None,
        },
        Shape {
            left: vec![(0, 0), (1, 2), (2, 2)],
            right: vec![(0, 2), (1, 2), (2, 2)],
            extra: vec![(0, 1)],
            bottom: vec![(0, 0), (0, 1), (0, 2)],
            cord: None,
        },
        Shape {
            left: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            right: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            extra: vec![],
            bottom: vec![(0, 0)],
            cord: None,
        },
        Shape {
            left: vec![(0, 0), (1, 0)],
            right: vec![(0, 1), (1, 1)],
            extra: vec![],
            bottom: vec![(0, 0), (0, 1)],
            cord: None,
        },
    ];
    let moves = include_str!("../input.txt").lines().nth(0).unwrap();
    let mut drawn = vec![vec![false; 7]; 3];

    let mut store = HashMap::<(usize, u8, String), (usize, usize)>::new();
    //
    let mut next = 0;
    let mut bottom = 0;

    //
    let mut obj = get_shape(&shapes, &mut next, bottom);
    new_lines(&mut drawn, bottom + 6);
    let moves = moves.chars().collect::<Vec<char>>();
    //
    let mut _ind = 0;
    let mut done = false;
    let mut height_extra = 0;
    loop {
        let action = moves[_ind];
        _ind = (_ind + 1) % moves.len();
        //
        if next == no_of_rocks + 1 {
            // to be changed for part 2
            break;
        }
        match action {
            '>' => {
                if obj.can_shift(&drawn, 1) {
                    obj.shift(1);
                }
            }
            '<' => {
                if obj.can_shift(&drawn, -1) {
                    obj.shift(-1);
                }
            }
            _ => {}
        }
        if obj.can_fall(&drawn) {
            obj.fall();
        } else {
            obj.draw(&mut drawn);
            bottom = usize::max(bottom, obj.top() + 1);
            // fast track
            let last_rows = get_last_rows(&drawn, 30);
            let details = (_ind, (next % 5) as u8, last_rows);
            if !done && store.get(&details).is_some() {
                let prev = store.get(&details).unwrap();
                let rock_per_cycle = next - prev.0;
                let h_per_cycle = bottom - prev.1;
                let cycles = (no_of_rocks - next) / rock_per_cycle;
                //
                height_extra = cycles * h_per_cycle;
                next += rock_per_cycle * cycles;
                done = true;
            }
            store.insert(details, (next, bottom));
            //
            new_lines(&mut drawn, bottom + 6);
            obj = get_shape(&shapes, &mut next, bottom);
        }
    }
    println!("{}", bottom + height_extra);
}

fn get_last_rows(m: &Vec<Vec<bool>>, last: i32) -> String {
    let mut s = String::new();
    for x in i32::max(0, m.len() as i32 - last) as usize..m.len() {
        let ss = m[x].iter().for_each(|&x| {
            if x {
                s.push('#');
            } else {
                s.push('.');
            }
        });
    }
    s
}
fn get_shape(ss: &Vec<Shape>, next: &mut usize, bottom: usize) -> Shape {
    let mut obj = ss[*next % 5].clone();
    obj.cord.replace((bottom + 3, 2));
    *next += 1;
    obj
}

fn new_lines(drawn: &mut Vec<Vec<bool>>, needed: usize) {
    for _ in 0..(needed - drawn.len()) {
        drawn.push(vec![false; 7]);
    }
}

#[derive(Clone)]
struct Shape {
    left: Vec<(usize, usize)>,
    extra: Vec<(usize, usize)>,
    right: Vec<(usize, usize)>,
    bottom: Vec<(usize, usize)>,
    cord: Option<(usize, usize)>,
}

impl Shape {
    fn can_shift(&self, m: &Vec<Vec<bool>>, shift: i64) -> bool {
        let starting = self.cord.as_ref().unwrap();
        if shift == 1 {
            let mut can_move = true;
            for i in self.right.iter() {
                let right = i.1 + starting.1 + 1;
                if i.0 + starting.0 < m.len() {
                    can_move &= (right < 7 && !m[i.0 + starting.0][right]);
                }
            }
            can_move
        } else {
            let mut can_move = true;
            for i in self.left.iter() {
                let left = (i.1 + starting.1) as i64 - 1;
                if i.0 + starting.0 < m.len() {
                    can_move &= (left >= 0 && !m[i.0 + starting.0][left as usize]);
                }
            }
            can_move
        }
    }

    fn can_fall(&self, m: &Vec<Vec<bool>>) -> bool {
        let starting = self.cord.as_ref().unwrap();
        let mut can_move = true;
        for i in self.bottom.iter() {
            let bottom: i64 = (i.0 + starting.0) as i64 - 1;
            if bottom < m.len() as i64 {
                can_move &= (bottom >= 0 && !m[bottom as usize][i.1 + starting.1]);
            }
        }
        can_move
    }
    fn fall(&mut self) {
        let mut x = self.cord.as_mut().unwrap();
        x.0 -= 1;
    }
    fn shift(&mut self, shift: i64) {
        let mut x = self.cord.as_mut().unwrap();
        x.1 = (x.1 as i64 + shift) as usize;
    }

    fn draw(&mut self, m: &mut Vec<Vec<bool>>) {
        let starting = self.cord.as_ref().unwrap();
        for i in self.left.iter() {
            m[i.0 + starting.0][i.1 + starting.1] = true;
        }
        for i in self.extra.iter() {
            m[i.0 + starting.0][i.1 + starting.1] = true;
        }
        for i in self.right.iter() {
            m[i.0 + starting.0][i.1 + starting.1] = true;
        }
    }

    fn top(&self) -> usize {
        return self.right.iter().map(|(x, _)| x).max().unwrap() + self.cord.as_ref().unwrap().0;
    }
}

fn draw_m(m: &Vec<Vec<bool>>) {
    println!("------");
    m.iter().rev().for_each(|x| {
        let string = x
            .iter()
            .map(|&e| if e { '#' } else { '.' })
            .collect::<String>();
        println!("{string}");
    })
}
