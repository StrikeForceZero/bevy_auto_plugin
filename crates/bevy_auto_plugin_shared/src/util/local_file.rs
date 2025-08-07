use thiserror::Error;

#[derive(Error, Debug)]
pub enum LocalFileError {
    /// `Span::local_file()` came back `None` – the span is virtual or remapped.
    #[error("span does not refer to a real on‑disk file")]
    VirtualSpan,

    /// Something went wrong while determining if called from rustc.
    #[error(transparent)]
    RustcDetection(#[from] crate::util::env::RustcDetectionError),
}

pub enum LocalFile {
    File(String),
    #[cfg(feature = "lang_server_noop")]
    Noop,
    Error(LocalFileError),
}

/// Panics if called from outside a procedural macro.
///
/// TODO: remove when rust-analyzer fully implements local_file https://github.com/rust-lang/rust/blob/4e973370053a5fe87ee96d43c506623e9bd1eb9d/src/tools/rust-analyzer/crates/proc-macro-srv/src/server_impl/rust_analyzer_span.rs#L144-L147
pub fn resolve_local_file() -> LocalFile {
    match crate::modes::flat_file::file_state::get_file_path() {
        Some(p) => LocalFile::File(p),
        None => {
            #[cfg(feature = "lang_server_noop")]
            {
                match crate::util::env::is_rustc() {
                    Ok(false) => return LocalFile::Noop,
                    Err(e) => return LocalFile::Error(e.into()),
                    _ => {} // fall through
                }
            }
            LocalFile::Error(LocalFileError::VirtualSpan)
        }
    }
}
