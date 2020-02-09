use crate::Input::*;
use anyhow::Result;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
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

#[derive(Debug, Clone)]
enum Input {
    File { count: usize, data: Vec<String> },
    Io,
}

impl Input {
    fn new(path: Option<impl AsRef<Path>>) -> Result<Self> {
        match path {
            Some(p) => {
                let s = read_to_string(&p)?;
                Ok(Self::File {
                    count: 0,
                    data: s.lines().map(String::from).collect(),
                })
            }
            None => Ok(Self::Io),
        }
    }
}

impl Iterator for Input {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::File { count: c, data: d } => {
                let r = d.get(*c).cloned();
                *c += 1;
                r
            }
            Self::Io => {
                let mut input = String::new();
                print!("//> ");
                io::stdout().flush().ok()?;
                io::stdin().read_line(&mut input).ok()?;
                Some(input)
            }
        }
    }
}

struct Main {
    passwords: Input,
    users: Input,
    cmd: String,
    count: usize,
}

impl Main {
    fn new(passwords: Option<PathBuf>, users: Option<PathBuf>, cmd: String) -> Self {
        let passwords = Input::new(passwords).unwrap();
        let users = Input::new(users).unwrap();
        Self {
            passwords,
            users,
            cmd,
            count: 0,
        }
    }

    fn ussrp3n(&self, user: &str, pass: &str) -> Result<Output> {
        Ok(Command::new("sh")
            .arg("-c")
            .arg(self.cmd.replace("{user}", user).replace("{password}", pass))
            .output()?)
    }
}

impl Iterator for Main {
    type Item = Result<Output>;

    fn next(&mut self) -> Option<Self::Item> {
        //TODO figure out what should be done here...
        Some(match (&self.users, &self.passwords) {
            (File { .. }, File { .. }) => self.ussrp3n("nig", "ger"),
            (Io, Io) => self.ussrp3n(
                &self.users.clone().next().unwrap(),
                &self.passwords.next().unwrap(),
            ),
            (File { .. }, Io) => self.ussrp3n("nig", "ger"),
            (Io, File { .. }) => self.ussrp3n("nig", "ger"),
        })
    } //function end
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
