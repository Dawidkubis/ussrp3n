use std::fs::read_to_string;
use std::path::PathBuf;
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

fn main() {
    let opt = Opt::from_args();

    let command = read_to_string(opt.command).unwrap();

    let cmd = Command::new("sh").arg("-c").arg(command).output().unwrap();

    let ip = String::from_utf8(cmd.stdout).unwrap();
    let err = String::from_utf8(cmd.stderr).unwrap();

    println!("{}\n{}", err, ip);
}
