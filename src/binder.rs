use std::{fs, io, path::Path};

use ignore::WalkBuilder;

use crate::document::{Document, MetaData, SrcFile};

pub fn collect_src_files(target_dir: &Path) -> io::Result<Document> {
    let mut file_paths = Vec::new();
    let walker = WalkBuilder::new(target_dir).build();

    let title = target_dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    for result in walker {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.path().is_file() {
            let rel_path = entry
                .path()
                .strip_prefix(target_dir)
                .unwrap_or(entry.path())
                .display()
                .to_string();
            file_paths.push((rel_path, entry.into_path()));
        }
    }

    file_paths.sort_by(|a, b| a.0.cmp(&b.0));

    let mut files: Vec<SrcFile> = Vec::new();

    for (rel_path, full_path) in &file_paths {
        let ext = full_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("text");
        if let Ok(content) = fs::read_to_string(&full_path) {
            files.push(SrcFile::new(rel_path.clone(), ext.into(), content));
        }
    }

    let index = file_paths.into_iter().map(|(a, _)| a).collect();

    Ok(Document::new(MetaData::new(title), index, files))
}
