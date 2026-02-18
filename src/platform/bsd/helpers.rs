use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub fn locate_process(process_name: &str) -> Option<PathBuf> {
    let path_env = env::var("PATH").ok()?;

    for dir in path_env.split(':') {
        let dir = if dir.is_empty() { "." } else { dir };

        let dir = dir.trim_end_matches('/');

        let full_path = Path::new(dir).join(process_name);

        if full_path.is_file() && is_executable(&full_path) {
            return Some(full_path);
        }
    }

    None
}

fn is_executable(path: &Path) -> bool {
    fs::metadata(path)
        .map(|metadata| {
            let permissions = metadata.permissions();
            permissions.mode() & 0o111 != 0
        })
        .unwrap_or(false)
}
