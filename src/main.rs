use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Opt {
	/// the post's headers
	headers: PathBuf,
	
	/// the post's body, with {user} and {password} for replacement
	body: String,

	/// url to attack
	url: String,

	/// users list
	users: Option<PathBuf>,
	
	/// password dictionary
	passwords: Option<PathBuf>,

	/// proxy
	proxy: Option<String>,

	/// number of threads to run on
	#[structopt(short, long, default_value = "1")]
	threads:usize,
}

fn main() {
	let opt = Opt::from_args();
	let proxy = reqwest::Proxy::all("socks5h://localhost:9050").expect("tor isn't running");
	let client = reqwest::Client::builder()
		.proxy(proxy);

}
