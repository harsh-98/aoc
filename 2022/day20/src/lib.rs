use std::{cell::RefCell, rc::Rc};
pub fn solve(depkey: i64, iterations: usize) {
    let eles = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut v = Vec::new();
    for i in eles.iter() {
        let node = Rc::new(RefCell::new(Node {
            done: false,
            ele: *i * depkey,
            left: None,
            right: None,
        }));
        if v.len() == 0 {
            v.push(node);
        } else {
            let left = v.last().unwrap();
            left.borrow_mut().right = Some(node.clone());
            node.borrow_mut().left = Some(left.clone());
            v.push(node);
        }
    }
    let last = v.last().unwrap();
    let head = v[0].clone();
    head.borrow_mut().left = Some(last.clone());
    last.borrow_mut().right = Some(head);
    //
    for _ in 0..iterations {
        mix(&v);
    }

    //
    let new_e = print_arr(&v);
    let ans = [1000, 2000, 3000]
        .iter()
        .map(|x| new_e[x % new_e.len()])
        .sum::<i64>();
    println!("{ans}");
    //
}

fn mix(v: &Vec<Rc<RefCell<Node>>>) {
    let length = v.len() - 1;
    for node in v.iter() {
        // let mut g = node.borrow_mut();
        let mut left = node.borrow_mut().left.take().unwrap();
        let mut right = node.borrow_mut().right.take().unwrap();
        left.borrow_mut().right = Some(right.clone());
        right.borrow_mut().left = Some(left.clone());
        //
        let mut copy = node.borrow().ele % length as i64;
        while copy != 0 {
            if copy > 0 {
                let _r = right.clone().borrow().right.as_ref().unwrap().clone();
                right = _r;
                copy -= 1;
                if copy == 0 {
                    left = right.borrow_mut().left.take().unwrap();
                }
            } else {
                let _l = left.borrow().left.as_ref().unwrap().clone();
                left = _l;
                copy += 1;
                if copy == 0 {
                    right = left.borrow_mut().right.take().unwrap();
                }
            }
        }
        if copy == 0 {
            right.borrow_mut().left = Some(node.clone());
            node.borrow_mut().right = Some(right);
            node.borrow_mut().left = Some(left.clone());
            left.borrow_mut().right = Some(node.clone());
        }
        //
    }
}

fn print_arr(v: &Vec<Rc<RefCell<Node>>>) -> Vec<i64> {
    let mut zero = v.iter().find(|x| x.borrow().ele == 0).unwrap().clone();

    let mut new_e = Vec::new();

    while !zero.borrow().done {
        new_e.push(zero.borrow().ele);
        zero.borrow_mut().done = true;
        let new_node = zero.borrow().right.as_ref().unwrap().clone();
        zero = new_node;
    }
    v.iter().for_each(|x| x.borrow_mut().done = false);
    //
    println!("{:?}", new_e);
    new_e
}

struct Node {
    done: bool,
    ele: i64,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}
