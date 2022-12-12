use regex::Regex;
use serde::{Deserialize, Serialize, *};
use std::{cell::RefCell, rc::Rc};
pub fn solve() -> Vec<i32> {
    let dre = Regex::new(r"cd (\S+)").unwrap();
    let start = Rc::new(RefCell::new(Dir::new("start".to_owned(), None)));
    let mut cur = start.clone();
    for i in include_str!("../input.txt").lines() {
        match i.chars().nth(0).unwrap() {
            '$' => {
                let e = dre.captures_iter(i).nth(0);
                match e {
                    Some(e) => {
                        match &e[1] {
                            ".." => {
                                // cd ..
                                let x = cur.borrow_mut().parent.take().unwrap();
                                cur = x;
                                // println!("{}", cur.borrow().name);
                            }

                            e => {
                                // cd dir
                                let new_dir = Dir::new(e.to_owned(), Some(cur.clone()));
                                let new_dir = Rc::new(RefCell::new(new_dir));
                                cur.borrow_mut().dirs.push(new_dir.clone());
                                cur = new_dir;
                            }
                        }
                    }
                    None => { // ls
                    }
                }
            }
            _ => {
                let (t, name) = i.split_once(" ").unwrap();
                match t {
                    "dir" => (),
                    _ => {
                        cur.borrow_mut()
                            .files
                            .push(File::new(name.to_owned(), t.parse::<i32>().unwrap()));
                    }
                }
            }
        }
    }
    start.borrow_mut().total();
    start.borrow().dirs[0].borrow_mut().parent.take().unwrap();
    let vector = start.borrow().dirs[0]
        .borrow()
        .groups()
        .iter()
        .map(|s| s.size)
        .collect::<Vec<i32>>();
    vector
}

#[derive(Debug, Serialize, Deserialize)]
struct Dir {
    dirs: Vec<Rc<RefCell<Dir>>>,
    files: Vec<File>,
    size: Option<i32>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_some")]
    parent: Option<Rc<RefCell<Dir>>>,
}

#[derive(Debug)]
struct Group {
    name: String,
    size: i32,
}
impl Dir {
    fn new(name: String, parent: Option<Rc<RefCell<Dir>>>) -> Dir {
        return Dir {
            dirs: Vec::new(),
            files: Vec::new(),
            size: None,
            name: name,
            parent: parent,
        };
    }
    fn total(&mut self) -> i32 {
        if self.size == None {
            let mut sum_ = 0;
            sum_ += self
                .dirs
                .iter_mut()
                .map(|s| s.clone().borrow_mut().total())
                .sum::<i32>();
            sum_ += self.files.iter().map(|s| s.size).sum::<i32>();
            self.size = Some(sum_);
        }
        return self.size.unwrap_or(0);
    }
    fn groups(&self) -> Vec<Group> {
        let mut ans = Vec::new();
        for d in self.dirs.iter() {
            ans.extend(d.borrow().groups());
        }
        ans.push(Group {
            name: self.name.clone(),
            size: self.size.unwrap(),
        });
        return ans;
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct File {
    size: i32,
    name: String,
}

impl File {
    fn new(name: String, size: i32) -> File {
        return File {
            size: size,
            name: name,
        };
    }
}
