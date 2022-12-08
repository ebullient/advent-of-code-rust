use crate::puzzle_input;
use regex::Regex;
use std::collections::HashMap;

pub fn run() {
    let input: Vec<String> = puzzle_input::read_all_lines("./input/2022-d07-input.txt");
    let tree = dir_tree(&input);
    let sizes = tree.du();

    println!("** Part 1 Final: {:?}", filter_total(&sizes, 100000));
    println!(
        "** Part 2 Final: {:?}",
        free_candidate(&sizes, 70000000, 30000000)
    );
}

#[derive(Clone, Debug)]
struct DirTree {
    elements: Vec<Dir>,
    current: usize,
}
impl DirTree {
    pub fn new() -> DirTree {
        let mut elements: Vec<Dir> = Vec::new();
        elements.push(Dir::new(String::from("/"), None));

        DirTree {
            elements,
            current: 0,
        }
    }

    pub fn root(&mut self) -> &Dir {
        self.current = 0;
        self.elements.get(0).unwrap()
    }

    pub fn out(&mut self) -> &Dir {
        let pwd = &self.elements[self.current];
        if None == pwd.parent {
            panic!("{:?} doesn't have a parent", pwd.path);
        }
        self.current = pwd.parent.unwrap();
        &self.elements[self.current]
    }

    pub fn cd(&mut self, dir: &str) -> &Dir {
        let pwd = &self.elements[self.current];
        let next = pwd.dirs.get(dir);
        if None == next {
            panic!("{:?}: dir {:?} not found", pwd.path, dir);
        }
        self.current = *next.unwrap();
        &self.elements[self.current]
    }

    pub fn add_dir(&mut self, name: &str) {
        let idx = self.elements.len();
        let pwd = &mut self.elements[self.current];
        pwd.dirs.insert(name.to_string(), idx);

        let child = Dir::new(format!("{}{}/", pwd.path, name), Some(self.current));
        self.elements.push(child);
    }

    pub fn add_file(&mut self, name: &str, size: i32) {
        let pwd = &mut self.elements[self.current];
        pwd.files.insert(name.to_string(), size);
    }

    fn du(&self) -> HashMap<String, i32> {
        let mut result = HashMap::new();
        self.dir_size(&self.elements[0], &mut result);

        result
    }

    pub fn dir_size(&self, dir: &Dir, cache: &mut HashMap<String, i32>) -> i32 {
        let mut total = 0;
        for (_, size) in dir.files.iter() {
            total += size;
        }
        for (_, idx) in dir.dirs.iter() {
            let d = &self.elements[*idx];
            total += self.dir_size(d, cache);
        }
        cache.insert(dir.path.clone(), total);

        total
    }

    #[allow(dead_code)]
    fn dump(&self) {
        let root = &self.elements[0];
        self.print(root, "");
    }

    #[allow(dead_code)]
    fn print(&self, dir: &Dir, prefix: &str) {
        println!("{}+ {}", prefix, dir.path);
        let indent = prefix.to_owned() + "  ";
        for (name, size) in dir.files.iter() {
            println!("{}- {} ({})", indent, name, size);
        }
        for (_, idx) in dir.dirs.iter() {
            let d = &self.elements[*idx];
            self.print(d, &indent);
        }
    }
}

#[derive(Clone, Debug)]
struct Dir {
    path: String,
    parent: Option<usize>,
    dirs: HashMap<String, usize>,
    files: HashMap<String, i32>,
}
impl Dir {
    pub fn new(path: String, parent: Option<usize>) -> Dir {
        Dir {
            path: path.to_string(),
            parent,
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

fn dir_tree(input: &[String]) -> DirTree {
    lazy_static! {
        static ref CD: Regex = Regex::new(r"\$ cd (.*)").unwrap();
        static ref LS: Regex = Regex::new(r"\$ ls").unwrap();
        static ref FILE: Regex = Regex::new(r"(\d+) (.*)").unwrap();
        static ref DIR: Regex = Regex::new(r"dir (.*)").unwrap();
    }

    let mut dirtree = DirTree::new();
    let mut iter = input.iter();

    while let Some(line) = iter.next() {
        if line.is_empty() {
            continue;
        } else if CD.is_match(line) {
            let caps = CD.captures(line).unwrap();
            match &caps[1] {
                "/" => dirtree.root(),
                ".." => dirtree.out(),
                x => dirtree.cd(x),
            };
            // println!("CD: {:?}", dirtree.pwd());
        } else if DIR.is_match(line) {
            let caps = DIR.captures(line).unwrap();
            dirtree.add_dir(&caps[1]);
            // println!(" +- : {:?}", line);
        } else if FILE.is_match(line) {
            let caps = FILE.captures(line).unwrap();
            dirtree.add_file(&caps[2], (&caps[1]).parse::<i32>().unwrap());
            // println!(" : {:?}", line);
        } else if !line.starts_with("$") {
            // println!("LS: {:?}", line);
        }
    }
    dirtree
}

fn filter_total(sizes: &HashMap<String, i32>, filter: i32) -> i32 {
    sizes.values().map(|x| *x).filter(|x| *x < filter).sum()
}

fn free_candidate(sizes: &HashMap<String, i32>, total: i32, required: i32) -> i32 {
    let used = sizes.get("/").unwrap();
    let free = total - used;

    sizes
        .values()
        .map(|x| *x)
        .filter(|x| free + *x >= required)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = r#"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
            "#
        .split('\n')
        .map(|x| x.trim().to_string())
        .collect();

        let tree = dir_tree(&input);
        tree.dump();
        let sizes = tree.du();
        println!("{:?}", sizes);

        assert_eq!(*sizes.get("/a/e/").unwrap(), 584);
        assert_eq!(*sizes.get("/a/").unwrap(), 94853);
        assert_eq!(*sizes.get("/d/").unwrap(), 24933642);
        assert_eq!(*sizes.get("/").unwrap(), 48381165);

        assert_eq!(filter_total(&sizes, 100000), 95437);
        assert_eq!(free_candidate(&sizes, 70000000, 30000000), 24933642);
    }
}
