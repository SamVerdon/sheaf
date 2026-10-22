use std::{fs, io, path::PathBuf};

use ignore::WalkBuilder;

use crate::sheaf::{FrontMatter, Leaf, Sheaf};

pub fn collect_leaves(targets: &[PathBuf], include_hidden: bool) -> io::Result<Sheaf> {
    let is_project = targets.len() == 1 && targets[0].is_dir();
    let mut file_paths = Vec::new();
    let mut builder = WalkBuilder::new(&targets[0]);
    for target in targets.iter().skip(1) {
        builder.add(target);
    }
    let walker = builder.hidden(!include_hidden).build();

    for result in walker {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.path().is_file() {
            let rel_path = if is_project {
                entry
                    .path()
                    .strip_prefix(&targets[0])
                    .unwrap_or(entry.path())
                    .display()
                    .to_string()
            } else {
                entry.path().display().to_string()
            };
            file_paths.push((rel_path, entry.into_path()));
        }
    }

    file_paths.sort_by(|a, b| a.0.cmp(&b.0));

    let mut leaves: Vec<Leaf> = Vec::new();

    for (rel_path, full_path) in &file_paths {
        let ext = full_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("text");
        if let Ok(content) = fs::read_to_string(full_path) {
            leaves.push(Leaf::new(rel_path.clone(), ext.into(), content));
        }
    }

    if is_project {
        let title = &targets[0]
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let register = file_paths.into_iter().map(|(a, _)| a).collect();
        Ok(Sheaf::project(
            FrontMatter::new(title.clone()),
            register,
            leaves,
        ))
    } else {
        Ok(Sheaf::collection(leaves))
    }
}
