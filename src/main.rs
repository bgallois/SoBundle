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
    #[arg(short, long)]
    qt: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut linker = LinkerBuilder::new(args.exec);
    if let Some(qt) = args.qt {
        linker = linker.with_qt(qt);
    }
    let linker = linker.build();
    let appdir = AppDirBuilder::new(linker).build();
}
