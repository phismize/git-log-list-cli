mod system;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "phizemie", version="0.0.1", about="git log list cli", long_about = None)]
struct Args {
    #[clap(short, long)]
    repository_name: String,
}

fn main() {
    let args = Args::parse();
    let repository_name = args.repository_name.to_string();
    match &*repository_name {
        "001" => println!("001 => {}", "hello"),
        _ => println!("no select"),
    }
}