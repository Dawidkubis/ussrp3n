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
	#[structopt(short, long)]
	users: Option<PathBuf>,

	/// password dictionary
	#[structopt(short, long)]
	passwords: Option<PathBuf>,

	/// number of threads to run on
	#[structopt(short, long, default_value = "1")]
	threads: usize,
}

struct Main {
	passwords: Vec<String>,
	users: Vec<String>,
	cmd: String,
	count: usize,
}

impl Main {
	fn new(passwords: Vec<String>, users: Vec<String>, cmd: String) -> Self {
		Self {
			passwords,
			users,
			cmd,
			count: 0,
		}
	}

	fn ussrp3n(&self, user: String, pass: String) -> Result<Output> {
		Ok(Command::new("sh")
			.arg("-c")
			.arg(
				self.cmd
					.replace("{user}", user.as_ref())
					.replace("{password}", pass.as_ref()),
			)
			.output()?)
	}
}

impl Iterator for Main {
	type Item = Output;

	fn next(&mut self) -> Option<Self::Item> {
		let user = self.users.get(self.count % self.users.len())?.to_string();

		let pass = self
			.passwords
			.get(self.count / self.users.len() as usize)?
			.to_string();

		self.count += 1;

		println!("{} + {}", user, pass);
		Some(self.ussrp3n(user, pass).unwrap()) //TODO error check
	} //function end
}

enum Status {
	In,
	NotIn,
	Again,
}

struct Cache(Vec<(Output, Status)>);

impl Cache {
	fn new() -> Self {
		Self(vec![])
	}
}

fn main() {
	let opt = Opt::from_args();
	let cmd = read_to_string(opt.command).unwrap();
	let passwords: Vec<String> = read_to_string(opt.passwords.unwrap())
		.unwrap()
		.lines()
		.map(String::from)
		.collect();

	let users: Vec<String> = read_to_string(opt.users.unwrap())
		.unwrap()
		.lines()
		.map(String::from)
		.collect();

	let main = Main::new(passwords, users, cmd);

	let cache = Cache::new();

	// main loop
	for i in main {
		if !i.status.success() {}

		println!("{:?}", i);
	}
}
