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
struct Directory<'a> {
    name: &'a str,
    subdirs: Vec<Directory<'a>>,
    files: Vec<File>,
    total_size: usize,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str) -> Self {
        Self { name, subdirs: Vec::new(), files: Vec::new(), total_size: usize::MAX }
    }

    fn from_command_outputs(outputs: Vec<CommandResult<'a>>) -> Self {
        let mut root = Directory::new("/");
        let mut cwd: Option<&mut Directory> = None;
        let mut dirstack = Vec::<&'a str>::new();

        for cmd in outputs {
            match cmd {
                CommandResult::Cd(path) if path == ".." => {
                    dirstack.pop();
                    cwd = None;
                }
                CommandResult::Cd(path) if path == "/" => {
                    dirstack.clear();
                    cwd = None;
                }
                CommandResult::Cd(name) => {
                    cwd = cwd.map(|wd| wd.subdirs.iter_mut().find(|sub| sub.name == name).expect("subdir exists"));
                    dirstack.push(name);
                }
                CommandResult::ListResult(res) => {
                    // must reinit in either path to prove that no multiple mutable borrows may occur
                    if let Some(cwd_here) = cwd {
                        cwd_here.introduce(res);
                        cwd = Some(cwd_here)
                    } else {
                        let mut cwd_here = &mut root;
                        for name in &dirstack {
                            cwd_here = cwd_here.subdirs.iter_mut().find(|dir| &dir.name == name).expect("subdir present");
                        }
                        cwd_here.introduce(res);
                        cwd = Some(cwd_here);
                    }
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

    fn introduce(&mut self, results: Vec<LsEntry<'a>>) {
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
}

#[derive(Debug)]
enum LsEntry<'a> {
    File(File),
    Dir(Directory<'a>),
}

#[derive(Debug)]
enum CommandResult<'a> {
    Cd(&'a str),
    ListResult(Vec<LsEntry<'a>>),
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
        map(cmd_cd, |name| CommandResult::Cd(name)),
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

default_solution!(parse_input, solve_problem);