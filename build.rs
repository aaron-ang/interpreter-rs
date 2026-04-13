use std::{
    env,
    fmt::Write as _,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

fn discover_lox_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().and_then(|e| e.to_str()) == Some("lox") {
                    files.push(path);
                }
            }
        }
    }
    files.sort();
    files
}

fn fn_name_for(path: &Path) -> String {
    let without_ext = path.with_extension("");
    let rel = without_ext.to_string_lossy();
    let mut s = String::new();
    for ch in rel.chars() {
        if ch.is_ascii_alphanumeric() {
            s.push(ch.to_ascii_lowercase());
        } else {
            s.push('_');
        }
    }
    s
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let test_root = manifest_dir.join("test");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let files = discover_lox_files(&test_root);
    let mut out = String::new();
    out.push_str("// Auto-generated tests. Do not edit.\n");
    for file in files {
        let rel = file.strip_prefix(&manifest_dir).unwrap_or(&file);
        let rel_str = rel.to_string_lossy();
        let fn_name = fn_name_for(rel);
        let ignore = if rel_str.contains("/benchmark/") || rel_str.contains("/limit/") {
            "\n#[ignore]"
        } else {
            ""
        };
        let _ = write!(
            out,
            "{ignore}\n#[test]\nfn {fn_name}() {{ crate::test_support::run_lox_test(\"{rel_str}\"); }}\n"
        );
        println!("cargo:rerun-if-changed={rel_str}");
    }

    let out_file = out_dir.join("lox_generated_tests.rs");
    let mut f = File::create(&out_file).expect("create generated tests");
    f.write_all(out.as_bytes()).expect("write generated tests");
}
