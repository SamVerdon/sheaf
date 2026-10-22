use clap::Parser;
use ignore::WalkBuilder;
use std::fs;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

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

    let mut file_paths = Vec::new();
    let walker = WalkBuilder::new(&args.target_dir).build();

    for result in walker {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.path().is_file() {
            let rel_path = entry
                .path()
                .strip_prefix(&args.target_dir)
                .unwrap_or(entry.path())
                .display()
                .to_string();
            file_paths.push((rel_path, entry.into_path()));
        }
    }

    file_paths.sort_by(|a, b| a.0.cmp(&b.0));

    writeln!(writer, "## Structure\n")?;
    writeln!(writer, "```text`")?;
    for (rel_path, _) in &file_paths {
        writeln!(writer, "{}", rel_path)?;
    }
    writeln!(writer, "```\n`")?;

    writeln!(writer, "## Files\n")?;
    for (rel_path, full_path) in file_paths {
        let ext = full_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("text");

        if let Ok(content) = fs::read_to_string(&full_path) {
            writeln!(writer, "### `{}`", rel_path)?;
            writeln!(writer, "```{}\n{}\n```\n", ext, content)?;
        }
    }
    Ok(())
}
