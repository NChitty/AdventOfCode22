use aoc_runner_derive::aoc_generator;

pub mod file_system {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone)]
    pub enum FileSystemItem {
        File(usize),
        Directory(String, Rc<RefCell<Directory>>)
    }

    impl FileSystemItem {
        fn get_size(&self) -> usize {
            match self {
                FileSystemItem::File(size) => size.to_owned(),
                FileSystemItem::Directory(_, reference) => reference.clone().borrow_mut().get_size()
            }
        }
    }

    #[derive(Clone)]
    pub struct Directory {
        parent: Option<Rc<RefCell<Directory>>>,
        children: Vec<FileSystemItem>,
        total_size: usize
    }

    impl Directory {
        pub fn init() -> Self {
           Directory {
               parent: Option::None,
               children: Vec::new(),
               total_size: 0
           }
        }

        pub fn new(&mut self, name: String) -> () { 
            let child_dir = FileSystemItem::Directory(name, Rc::new(RefCell::new(Directory {
                parent: Option::Some(Rc::new(RefCell::new(self.clone()))),
                children: Vec::new(),
                total_size: 0
            })));

            self.children.push(child_dir);

        }
    
        pub fn get_size(&mut self) -> usize {
            if self.total_size == 0 {
                self.total_size = self.children.iter().map(|item| {
                    item.get_size()
                }).sum();
            }

            self.total_size
        }

        pub fn new_file(&mut self, size: usize) {
            self.children.push(FileSystemItem::File(size));
        }

        pub fn to_parent(&mut self) -> Option<Rc<RefCell<Self>>> {
            self.parent
        }

        pub fn change_dir(&self, name: String) -> Rc<RefCell<Self>> {
            for item in &self.children {
                match item {
                    FileSystemItem::Directory(dir, reference) => { 
                        if dir.eq(name.as_str()) {
                            return reference.clone();
                        }
                    },
                    _ => continue
               }
            }

            panic!("no such directory found")
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
use file_system::{Directory, FileSystemItem};

#[aoc_generator(day7, part1)]
fn parse_input(input: &str) -> Rc<RefCell<Directory>> {
    let root_dir = FileSystemItem::Directory(String::from("/"), Rc::new(RefCell::new(Directory::init())));
    let current_dir: Rc<RefCell<Directory>> = match root_dir {
        FileSystemItem::Directory(_, reference) => reference.clone(),
        _ => panic!("woops")
    };

    for line in input.lines() {
        // line is $ cd
          // change directory
        // dir name pattern, create new directory in current_dir
        // number -> push new file
    }

    current_dir.clone()
}
