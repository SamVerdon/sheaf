use crate::sheaf::Bind;
use clap::Parser;
use std::fs;
use std::io::{self, BufWriter};
use std::path::PathBuf;

mod binder;
mod sheaf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Flatten codebase for LLMs")]
struct Args {
    #[arg(default_value = ".")]
    target_dir: Vec<PathBuf>,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(long)]
    hidden: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let sheaf = binder::collect_leaves(&args.target_dir, args.hidden);
    if let Some(out_path) = &args.output {
        let output_file = fs::File::create(out_path)?;
        let mut writer = BufWriter::new(output_file);
        sheaf.bind(&mut writer)?;
    } else {
        let stdout = io::stdout();
        let mut writer = stdout.lock();
        sheaf.bind(&mut writer)?;
    }

    Ok(())
}
