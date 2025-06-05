use clap::Parser;

pub mod linker;
use linker::*;

pub mod appdir;
use appdir::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    exec: String,
    #[arg(short, long)]
    appdir: Option<String>,
}

fn main() {
    let args = Args::parse();
    let linker = LinkerBuilder::new(args.exec).build();
    //println!("{:?}", linker);
    let appdir = AppDirBuilder::new(linker).build();
}
