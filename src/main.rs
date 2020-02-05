use std::path::PathBuf;
use structopt::StructOpt;

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
    #[structopt(short, long)]
    proxy: Option<String>,

    /// number of threads to run on
    #[structopt(short, long, default_value = "1")]
    threads: usize,
}

fn main() {
    let opt = Opt::from_args();
    let proxy = reqwest::Proxy::all("socks5h://127.0.0.1:9050").expect("tor isn't running");
    let client = reqwest::blocking::Client::builder()
        .proxy(proxy)
        .build()
        .unwrap();

    let r = client.get("https://myexternalip.com/raw").send().unwrap();

    println!("{:?}", r.text());
}
