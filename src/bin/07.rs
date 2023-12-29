use advent_of_code_2022::read_file_input;

fn main(){
    let file = read_file_input("07.txt");
}

fn make_file_tree(input: &str) -> Directory{
    let root = Directory{
        name: String::from("/"),
        children: Box::new(Vec::new()),
        parent: Box::new(None),
    };

    let current : &Directory = &root;
    
    input.lines().map(|line| {
        if line.starts_with("$") && !line.contains("ls"){
            let test = line[2..line.len()].to_string().split_once(" ").unwrap();
            
        }
    });
}

enum Child{
    File(File),
    Directory(Directory),
}

struct File{
    name: String,
    size: u64,
    parent: Directory
}

struct Directory{
    name: String,
    children: Box<Vec<Child>>,
    parent: Box<Option<Directory>>,
}