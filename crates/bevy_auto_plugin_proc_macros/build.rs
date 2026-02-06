use std::{
    env,
    fs,
    path::{
        Path,
        PathBuf,
    },
};

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

fn verify_docs_gen_version(docs_gen: &Path, pkg_version: &str) {
    let version_path = docs_gen.join(".version");
    let version_contents = fs::read_to_string(&version_path).unwrap_or_else(|_| {
        panic!("docs_gen exists but is missing .version file at {}, run `./scripts/docs.sh` to resync them", version_path.display())
    });
    let version = version_contents.trim();
    if version != pkg_version {
        panic!(
            "docs_gen .version '{}' does not match crate version '{}', run `./scripts/docs.sh` to resync them",
            version, pkg_version
        );
    }
    println!("cargo:rerun-if-changed={}", version_path.display());
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing manifest dir"));
    let pkg_version = env::var("CARGO_PKG_VERSION").expect("missing package version");

    let workspace_docs = manifest_dir.join("../../docs");
    let docs_gen = manifest_dir.join("docs_gen");
    let docs_gen_docs = docs_gen.join("docs");

    if docs_gen.exists() {
        verify_docs_gen_version(&docs_gen, &pkg_version);
    }

    let source_docs = if workspace_docs.exists() {
        println!("cargo:rerun-if-changed={}", workspace_docs.display());
        workspace_docs
    } else if docs_gen_docs.exists() {
        println!("cargo:rerun-if-changed={}", docs_gen_docs.display());
        docs_gen_docs
    } else if docs_gen.exists() {
        println!("cargo:rerun-if-changed={}", docs_gen.display());
        docs_gen
    } else {
        panic!(
            "missing docs source: expected {} or {}",
            workspace_docs.display(),
            docs_gen.display()
        );
    };

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("missing OUT_DIR"));
    let out_docs = out_dir.join("docs");
    if out_docs.exists() {
        fs::remove_dir_all(&out_docs).expect("failed to clear OUT_DIR/docs");
    }
    copy_dir_all(&source_docs, &out_docs).expect("failed to copy docs into OUT_DIR");
}
