use clap::Parser;

pub mod linker;
use linker::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    exec: String,
}

fn main() {
    let args = Args::parse();
    let linker = LinkerBuilder::new(args.exec).build();
    println!("{:?}", linker);
}
