use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::rc::Rc;

trait Node {
    fn get_name(&self) -> &str;
    fn get_parent(&self) -> Option<Link>;
    fn get_size(&self) -> u64;
}

trait NodeTrait: Node + Debug {}

type Link = Rc<RefCell<Dir>>;

#[derive(Debug, Default)]
struct Dir {
    name: String,
    children: Vec<Link>,
    files: Vec<File>,
    parent: Option<Link>,
}

impl Dir {
    fn new(name: String, parent: Link) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Dir {
            name,
            children: Vec::new(),
            files: Vec::new(),
            parent: Some(parent),
        }))
    }
    fn add_child(&mut self, child: Link) {
        //check that child does not already exist
        for c in self.children.iter() {
            if c.borrow().get_name() == child.borrow().get_name() {
                return;
            }
        }
        self.children.push(child);
    }

    fn add_file(&mut self, file: File) {
        //check that file does not already exist
        for f in self.files.iter() {
            if f.get_name() == file.get_name() {
                return;
            }
        }
        self.files.push(file);
    }
}

impl Node for Dir {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_parent(&self) -> Option<Link> {
        self.parent.clone()
    }

    fn get_size(&self) -> u64 {
        let mut size = 0;
        for child in self.children.iter() {
            size += child.borrow().get_size();
        }
        for file in self.files.iter() {
            size += file.get_size();
        }
        size
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
    parent: Option<Link>,
}

impl File {
    fn new(name: String, size: u64) -> Self {
        File {
            name,
            size,
            parent: None,
        }
    }

    fn get_size(&self) -> u64 {
        self.size
    }

    fn get_parent(&mut self) -> &mut Option<Link> {
        &mut self.parent
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Node for File {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_parent(&self) -> Option<Link> {
        self.parent.clone()
    }

    fn get_size(&self) -> u64 {
        self.size
    }
}

fn breadth_first_map<F>(dir: Rc<RefCell<Dir>>, mut f: F)
where
    F: FnMut(Rc<RefCell<Dir>>),
{
    let mut queue = Vec::new();

    queue.push(dir.clone());

    while let Some(node) = queue.pop() {
        f(node.clone());
        for child in node.borrow().children.iter() {
            queue.push(child.clone());
        }
    }
}

fn parse_input(input: &str) -> Dir {
    let mut root = Dir::new(
        "/".to_string(),
        Rc::new(RefCell::new(Dir {
            name: "/".to_string(),
            children: Vec::new(),
            files: Vec::new(),
            parent: None,
        })),
    );
    let mut current_dir = root.clone();

    let mut ls_command = false;
    for line in input.lines() {
        if ls_command {
            match line {
                line if line.starts_with("dir") => {
                    let dir = line.split(" ").skip(1).next().unwrap();
                    println!("dir: {}", dir);
                    continue;
                }
                line if line.split(" ").count() == 2 => {
                    let size = line.split(" ").next().unwrap();
                    let name = line.split(" ").skip(1).next().unwrap();
                    println!("file: {} {}", size, name);

                    let file = File::new(name.to_string(), size.parse().unwrap());
                    current_dir.borrow_mut().add_file(file);
                    continue;
                }
                _ => {
                    ls_command = false;
                    println!("");
                    println!("{}", line);
                }
            }
        }
        match line {
            line if line.starts_with("$ cd") => {
                let mut path = line.split(" ").skip(2);
                let dir = path.next().unwrap();
                println!("cd: {}", dir);

                match dir {
                    "/" => {
                        current_dir = root.clone();
                    }
                    ".." => {
                        let parent = current_dir.borrow().get_parent().clone();
                        if let Some(parent) = parent {
                            current_dir = parent;
                        }
                    }
                    dir => {
                        let mut child_dir: Option<Link> = None;
                        for child in current_dir.borrow().children.iter() {
                            if child.borrow().get_name() == dir {
                                child_dir = Some(child.clone());
                                break;
                            }
                        }
                        if let Some(child_dir) = child_dir {
                            current_dir = child_dir;
                        } else {
                            let new_dir = Dir::new(dir.to_string(), current_dir.clone());

                            current_dir.borrow_mut().add_child(new_dir.clone());

                            // .add_child(new_dir.clone());
                            current_dir = new_dir.clone();
                        }
                    }
                }
            }
            line if line.starts_with("$ ls") => {
                ls_command = true;
                println!("ls");
            }

            line => {
                panic!("Invalid input: {}", line);
            }
        }
    }
    root.take()
}

fn part1() {
    println!("Day 7");

    //get input from file
    let input = include_str!("../input/7");

    let dirs = parse_input(input);
    let size = dirs.get_size();
    println!("Root size: {}", size);

    //visit all dirs
    let mut capped_size = 0;

    let test = Rc::new(RefCell::new(dirs));

    breadth_first_map(test, |c: Link| {
        let size = c.borrow().get_size();

        if size <= 100000 {
            println!("dir: {:?} size: {}", c.borrow().get_name(), size);
            capped_size += size;
        }
    });

    println!("Capped size: {}", capped_size);
}

pub fn run() {
    part1();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cd() {
        let input = "$ cd /\n$ cd a";
        let dirs = parse_input(input);

        println!("dirs: {:?}", dirs);
        assert_eq!(dirs.get_name(), "/");
        assert_eq!(dirs.children.len(), 1);
        assert_eq!(dirs.children[0].borrow().get_name(), "a");
    }

    #[test]
    fn test_capped_size() {
        let input = include_str!("../input/test7");

        let dirs = parse_input(input);
        let size = dirs.get_size();
        println!("Root size: {}", size);

        //visit all dirs
        let mut capped_size = 0;

        let test = Rc::new(RefCell::new(dirs));

        breadth_first_map(test, |c: Link| {
            let size = c.borrow().get_size();

            println!("dir: {:?} size: {}", c.borrow().get_name(), size);
            if size <= 100000 {
                println!("dir: {:?} size: {}", c.borrow().get_name(), size);
                capped_size += size;
            }
        });
        println!("Capped size: {}", capped_size);
        assert_eq!(capped_size, 95437);
    }
}
