use clap::Parser;

const SKIP_LIBS: [&str; 9] = [
    "libc.so",
    "ld-linux",
    "libm.so",
    "libpthread.so",
    "libdl.so",
    "librt.so",
    "libnsl.so",
    "libutil.so",
    "libresolv.so",
];

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
    #[arg(short, long)]
    bundle: bool,
    #[arg(long = "exclude")]
    exclude_libs: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let mut skip_libs: Vec<String> = SKIP_LIBS.iter().map(|s| s.to_string()).collect();
    skip_libs.extend(args.exclude_libs);

    let mut linker = LinkerBuilder::new(args.exec).with_skip_libs(skip_libs);
    if let Some(qt) = args.qt {
        linker = linker.with_qt(qt);
    }
    let linker = linker.build();
    let mut appdir = AppDirBuilder::new(linker, args.appdir);
    if args.bundle {
        appdir = appdir.with_bundle();
    }
    let appdir = appdir.build();
    appdir.check_rpath();
}
