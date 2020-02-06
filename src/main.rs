use std::path::PathBuf;
use std::fs::read_to_string;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// the file with curl command
    curl: PathBuf,

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

    let command = dbg!(read_to_string(opt.curl)).unwrap();

	let ip = Command::new("curl")
		.args(command.split_ascii_whitespace())
		.output()
		.unwrap()
		.stdout;

	// FIXME this is a very very bad
	
	let ip = String::from_utf8(ip).unwrap();

	println!("{}", ip);

}
