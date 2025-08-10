use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustcDetectionError {
    #[error("could not query current executable: {source}")]
    CurrentExe {
        #[from]
        source: std::io::Error,
    },
    #[error("env::current_exe() path has no file‑name component: {path}")]
    NoFileName { path: std::path::PathBuf },
}

/// Checks if the current executable looks like rustc
pub fn is_rustc() -> Result<bool, RustcDetectionError> {
    use std::env;
    use std::ffi::OsStr;
    let exe = env::current_exe().map_err(RustcDetectionError::from)?;
    let Some(stem) = exe.file_stem().and_then(OsStr::to_str) else {
        return Err(RustcDetectionError::NoFileName { path: exe });
    };
    Ok(stem.eq_ignore_ascii_case("rustc"))
}
