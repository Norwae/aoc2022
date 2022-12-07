use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many0, many1};
use nom::sequence::{preceded, separated_pair, terminated, tuple};

use crate::util::{default_solution, parse_usize};

#[derive(Debug)]
struct File {
    size: usize,
}

#[derive(Debug)]
struct Directory {
    name: String,
    subdirs: Vec<Directory>,
    files: Vec<File>,
    total_size: usize,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self { name: name.to_string(), subdirs: Vec::new(), files: Vec::new(), total_size: usize::MAX }
    }

    fn from_command_outputs(outputs: Vec<CommandResult>) -> Self {
        let mut root = Directory::new("/");
        let mut cwd = &mut root;
        let mut dirstack = Vec::<String>::new();

        for cmd in outputs {
            match cmd {
                CommandResult::Cd(ref path) if path == ".." => {
                    dirstack.pop();
                    cwd = root.subdir_deep(&dirstack);
                }
                CommandResult::Cd(ref path) if path == "/" => {
                    dirstack.clear();
                    cwd = &mut root;
                }
                CommandResult::Cd(name) => {
                    cwd = cwd.subdirs.iter_mut().find(|sub| sub.name == name.as_str()).expect("subdir exists");
                    dirstack.push(name);
                }
                CommandResult::ListResult(res) => {
                    cwd.introduce(res)
                }
            }
        }

        root.finalize();
        root
    }

    fn traverse_subdirs<Visitor: FnMut(&Directory)>(&self, visit: &mut Visitor) {
        visit(self);
        for subdir in &self.subdirs {
            subdir.traverse_subdirs(visit);
        }
    }

    fn introduce(&mut self, results: Vec<LsEntry>) {
        assert_eq!(self.total_size, usize::MAX, "size calculation not yet done");

        for entry in results {
            match entry {
                LsEntry::File(f) => self.files.push(f),
                LsEntry::Dir(d) => {
                    self.subdirs.push(d);
                }
            }
        }
    }

    fn finalize(&mut self) {
        let files: usize = self.files.iter().map(|f| f.size).sum();
        let subdirs: usize = self.subdirs.iter_mut().map(|d| {
            d.finalize();
            d.total_size
        }).sum();
        self.total_size = files + subdirs;
    }

    fn subdir_deep(&mut self, path: &[String]) -> &mut Directory {
        if path.is_empty() {
            self
        } else {
            let local_path = &path[0];
            let next = self.subdirs.iter_mut().find(|d| &d.name == local_path).expect("subdir known");
            next.subdir_deep(&path[1..])
        }
    }
}

#[derive(Debug)]
enum LsEntry {
    File(File),
    Dir(Directory),
}

#[derive(Debug)]
enum CommandResult {
    Cd(String),
    ListResult(Vec<LsEntry>),
}

fn namelike(input: &str) -> IResult<&str, &str> {
    take_while(|ch: char| !ch.is_ascii_whitespace())(input)
}

fn cmd_ls(input: &str) -> IResult<&str, &str> {
    terminated(tag("$ ls"), line_ending)(input)
}

fn cmd_cd(input: &str) -> IResult<&str, &str> {
    map(terminated(tuple((tag("$ cd "), namelike)), line_ending), |(_, str)| str)(input)
}

fn file_entry(input: &str) -> IResult<&str, LsEntry> {
    map(separated_pair(parse_usize, tag(" "), namelike), |(size, _)| LsEntry::File(File { size }))(input)
}

fn dir_entry(input: &str) -> IResult<&str, LsEntry> {
    map(tuple((tag("dir "), namelike)), |(_, name): (&str, &str)| LsEntry::Dir(Directory::new(name)))(input)
}

fn ls_output(input: &str) -> IResult<&str, Vec<LsEntry>> {
    many0(
        terminated(
            alt((file_entry, dir_entry)),
            line_ending,
        )
    )(input)
}

fn cmd(input: &str) -> IResult<&str, CommandResult> {
    alt((
        map(cmd_cd, |name| CommandResult::Cd(name.to_string())),
        map(preceded(cmd_ls, ls_output), |entries| CommandResult::ListResult(entries))
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<CommandResult>> {
    many1(cmd)(input)
}

fn solve_problem(input: Vec<CommandResult>) {
    let root = Directory::from_command_outputs(input);

    let mut small_subdirs = 0usize;
    root.traverse_subdirs(&mut |dir: &Directory| {
        let size = dir.total_size;
        if size < 100000 {
            small_subdirs += size
        }
    });
    println!("Part1: {}", small_subdirs);
    const TOTAL: usize = 70000000;
    const REQUIRED: usize = 30000000;
    let available = TOTAL - root.total_size;
    let still_required = REQUIRED - available;

    let mut delete_dir_size = usize::MAX;

    root.traverse_subdirs(&mut |dir: &Directory| {
        let size = dir.total_size;
        if size >= still_required && size < delete_dir_size {
            delete_dir_size = size
        }
    });
    println!("Part2: {}", delete_dir_size);
}

pub fn solve() {
    default_solution(parse_input, |results| {
        solve_problem(results)
    })
}