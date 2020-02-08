use anyhow::Result;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// the file with curl command
    command: PathBuf,

    /// users list
    users: Option<PathBuf>,

    /// password dictionary
    passwords: Option<PathBuf>,

    /// number of threads to run on
    #[structopt(short, long, default_value = "1")]
    threads: usize,
}

#[derive(Debug)]
struct FileInput {
    count: usize,
    data: Vec<String>,
}

impl FileInput {
    fn new(path: impl AsRef<Path>) -> Result<Self> {
        let s = read_to_string(&path)?;
        Ok(Self {
            count: 0,
            data: s.lines().map(String::from).collect(),
        })
    }
}

impl Iterator for FileInput {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.data.get(self.count).cloned();
        self.count += 1;
        r
    }
}

#[derive(Debug)]
struct Input;

impl Input {
    fn new() -> Self {
        Self // :D
    }
}

impl Iterator for Input {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut input = String::new();
        print!("//> ");
        io::stdout().flush().ok()?;
        io::stdin().read_line(&mut input).ok()?;
        Some(input)
    }
}

type I = dyn Iterator<Item = String>;
struct Main {
    passwords: Box<I>,
    users: Box<I>,
    cmd: String,
}

impl Main {
    fn new(passwords: Option<PathBuf>, users: Option<PathBuf>, cmd: String) -> Self {
        let passwords: Box<I> = match passwords {
            Some(s) => Box::new(FileInput::new(s).unwrap()),
            None => Box::new(Input::new()),
        };
        let users: Box<I> = match users {
            Some(s) => Box::new(FileInput::new(s).unwrap()),
            None => Box::new(Input::new()),
        };
        Self {
            passwords,
            users,
            cmd,
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    let cmd = read_to_string(opt.command).unwrap();

    let main = Main::new(opt.passwords, opt.users, cmd);

    //let cmd = Command::new("sh").arg("-c").arg(command).output().unwrap();

    //let ip = String::from_utf8(cmd.stdout).unwrap();
    //let err = String::from_utf8(cmd.stderr).unwrap();

    //println!("{}\n{}", err, ip);
}
