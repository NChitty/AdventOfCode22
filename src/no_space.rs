use aoc_runner_derive::aoc_generator;
use regex::Regex;

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

pub enum Path {
  File(usize),
  Folder(HashMap<String, Rc<RefCell<Path>>>),
}

impl Path {
  fn new_folder() -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self::Folder(HashMap::new())))
  }

  fn add_file(&mut self, name: &str, size: usize) {
    match self {
      Self::File(_) => panic!("Cannot add node to a file"),
      Self::Folder(children) => {
        children.insert(name.to_owned(), Rc::new(RefCell::new(Self::File(size))))
      }
    };
  }

  fn get_folder(&mut self, name: &str) -> Rc<RefCell<Self>> {
    match self {
      Self::File(_) => panic!("Cannot retrieve a folder from file"),
      Self::Folder(children) => (*children)
          .entry(name.to_owned())
          .or_insert_with(Self::new_folder)
          .clone(),
    }
  }

  fn get_size(&mut self) -> (usize, Vec<usize>) {
    match self {
      Path::File(size) => (*size, vec![]),
      Path::Folder(children) => {
        let mut total_size = 0;
        let mut total_size_list = Vec::new();

        for child in children.values() {
          let mut child = child.deref().borrow_mut();
          let (child_total_size, child_total_size_list) = child.get_size();
          total_size += child_total_size;
          total_size_list.extend(child_total_size_list);
        }

        total_size_list.push(total_size);
        (total_size, total_size_list)
      }
    }
  }
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Rc<RefCell<Path>> {
  let root = Path::new_folder();
  let mut cwd = vec![root.clone()];
  let cd_re = Regex::new(r"^\$ cd (?<dir>/|\.\.|\w+)$").unwrap();
  let ls_file_re = Regex::new(r"^(?<file_size>\d+) (?<file_name>\w+\.*\w*)$").unwrap();

  for line in input.lines() {
    let cd_cap = cd_re.captures(line);
    let ls_file_cap = ls_file_re.captures(line);

    if let Some(cap) = cd_cap {
      match &cap["dir"] {
        "/" => {
          cwd = vec![root.clone()];
        }
        ".." => {
          cwd.pop();
        }
        dir_name => {
          let folder = cwd.last().unwrap().as_ref().borrow_mut().get_folder(dir_name);
          cwd.push(folder);
        }
      }
      continue;
    }

    // number -> push new file
    if let Some(cap) = ls_file_cap {
      let size: usize = (&cap["file_size"]).parse().unwrap();
      cwd.last()
          .unwrap()
          .as_ref()
          .borrow_mut()
          .add_file(
            &cap["file_name"],
            size
          );
    }
  }

  root
}

#[aoc(day7, part1)]
fn sum_less_100k(input: &Rc<RefCell<Path>>) -> usize {
  let (_, size_list) = (*input).deref().borrow_mut().get_size();
  return size_list.iter().filter(|x| **x <= 100_000).sum();
}

#[aoc(day7, part2)]
fn free(input: &Rc<RefCell<Path>>) -> usize {
  let (total_size, size_list) = (*input).deref().borrow_mut().get_size();
  let must_free = total_size - 40_000_000;
  return size_list.iter().filter(|x| **x >= must_free).min().unwrap().to_owned();
}
