use crate::sheaf::Render;
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
    target_dir: PathBuf,

    #[arg(short, long, default_value = "context.md")]
    output: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let output_file = fs::File::create(&args.output)?;
    let mut writer = BufWriter::new(output_file);

    let doc = collector::collect_src_files(&args.target_dir)?;
    doc.render(&mut writer)?;

    Ok(())
}
