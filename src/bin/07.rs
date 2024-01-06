use advent_of_code_2022::read_file_input;

fn main(){
    let file = read_file_input("07.txt");

    println!("Total size of small dirs: {}", sizes);
}

fn make_file_tree(input: &str) -> Directory{
    let mut tree = Tree::new();
    
    input.lines().for_each(|line| {
        match line.split_once(" ").unwrap(){
            ("$", command) => {
                match command.split_once(" ") {
                    Some(command) => {
                        match command.1 {
                            "/" => {
                                tree.stack.clear();
                                tree.stack.push(tree.root.clone());
                                println!("Back to root")
                            },
                            ".." => {
                                tree.stack.pop();
                                println!("Move up one dir, new current: {}", tree.stack.last().unwrap().name);
                            },
                            name => {
                                // println!("Try move to dir: {}", name);
                                let current = tree.stack.last().unwrap().clone();
                                for i in current.children.iter(){
                                    match i {
                                        Child::Directory(d) => {
                                            // println!("Found dir: {}", d.name);
                                            if d.name == name {
                                                println!("New current: {}", d.name);
                                                tree.stack.push(d.clone())

                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            },
            ("dir", name) => {
                println!("New dir: {}", name);
                tree.add_child(Child::Directory(Directory{
                    name: String::from(name),
                    children: Box::new(Vec::new()),
                }));
            },
            (size, name) => {
                if let Ok(size) = size.parse::<u64>(){

                    println!("New file: {} with size {}", name, size);
                    tree.add_child(Child::File(File{
                        name: String::from(name),
                        size,
                    }));
                }
            },
        }
        
    });

    tree.root
    
}


struct Tree {
    root: Directory,
    stack: Vec<Directory>,
}

impl Tree{
    fn new() -> Self{
        Tree{
            root: Directory{
                name: String::from("/"),
                children: Box::new(Vec::new()),
            },
            stack: Vec::new(),
        }
    }

    fn add_child(&mut self, child: Child){
        match child{
            Child::File(file) => {
                self.stack.last_mut().unwrap().children.push(Child::File(file));
            },
            Child::Directory(dir) => {
                self.stack.last_mut().unwrap().children.push(Child::Directory(dir))
            },
        }
    }
}

#[derive(Clone)]
enum Child{
    File(File),
    Directory(Directory),
}

#[derive(Clone)]
struct File{
    name: String,
    size: u64,
}


#[derive(Clone)]
struct Directory{
    name: String,
    children: Box<Vec<Child>>,
}

/*
    GetSize:

    Starting from root, go through all children
        if file, add file.size to size
        if dir -> add GetSize(dir)

    if size < 100000 -> push size to vec of small dirs
*/