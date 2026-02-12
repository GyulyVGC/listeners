#[cfg(target_os = "freebsd")]
fn main() {
    let src_dir = std::path::PathBuf::from("src/platform/freebsd/native");

    let mut c_files = Vec::new();
    find_c_files(&src_dir, &mut c_files);

    if c_files.is_empty() {
        println!("cargo:warning=no C files found in {:?}", src_dir);
        return;
    }

    let mut build = cc::Build::new();

    for file in &c_files {
        build.file(file);
    }

    build.include(src_dir.clone());

    build.compile("native_freebsd_lib");

    for file in &c_files {
        println!("cargo:rerun-if-changed={}", file.display());
    }

    println!("cargo:rerun-if-changed={}", src_dir.display());
}

#[cfg(not(target_os = "freebsd"))]
fn main() {}

#[cfg(target_os = "freebsd")]
fn find_c_files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    if !dir.is_dir() {
        return;
    }

    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if path.is_dir() {
            find_c_files(&path, out);
        } else if let Some(ext) = path.extension() 
            && ext == "c" {
            out.push(path);
        }
    }
}
