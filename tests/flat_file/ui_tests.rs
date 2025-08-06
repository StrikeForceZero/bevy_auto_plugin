use std::fmt::{Display, Formatter};

const RS_EXTENSION: &str = "rs";
const STDERR_EXTENSION: &str = "stderr";

enum SubDir {
    Root,
    #[cfg(not(feature = "legacy_path_param"))]
    Standard,
    #[cfg(feature = "legacy_path_param")]
    LegacyPathParam,
    Nightly,
    Stable,
}

impl SubDir {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Root => "",
            #[cfg(not(feature = "legacy_path_param"))]
            Self::Standard => "standard",
            #[cfg(feature = "legacy_path_param")]
            Self::LegacyPathParam => "legacy_path_param",
            Self::Nightly => "nightly",
            Self::Stable => "stable",
        }
    }
}

impl Display for SubDir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

fn current_source_dir<'a>() -> Option<&'a str> {
    use std::path::Path;
    Path::new(file!()).parent()?.to_str()
}

fn get_tests_path_string(sub_dir: SubDir) -> String {
    let current_source_dir = current_source_dir().expect("failed to get the current source dir");
    format!("{current_source_dir}/ui/{sub_dir}/")
}

fn get_trybuild_path_string(sub_dir: SubDir) -> String {
    let current_source_dir = current_source_dir().expect("failed to get the current source dir");
    format!("{current_source_dir}/ui/{sub_dir}/*.{RS_EXTENSION}")
}

mod utils {

    use std::fs::{DirEntry, File, read_dir};
    use std::io::{Read, Result};
    use std::path::{Path, PathBuf};

    fn is_extension_factory(extension: Option<&str>) -> impl Fn(&DirEntry) -> bool {
        move |entry: &DirEntry| {
            if let Some(extension) = extension {
                if entry.path().extension().unwrap_or_default() != extension {
                    return false;
                }
            }
            true
        }
    }

    fn is_file(entry: &DirEntry) -> bool {
        entry.path().is_file()
    }

    fn files_in_dir(
        path: impl AsRef<Path>,
        extension: Option<&str>,
    ) -> Result<impl Iterator<Item = DirEntry>> {
        println!("reading: {}", path.as_ref().display());
        if let Some(extension) = extension {
            println!("filtering extension: {extension}");
        }
        let rs_files = read_dir(path)?
            .filter_map(Result::ok) // ignore errors
            .filter(is_file)
            .filter(is_extension_factory(extension));
        Ok(rs_files)
    }

    pub fn count_files_in_dir(path: impl AsRef<Path>, extension: Option<&str>) -> Result<usize> {
        Ok(files_in_dir(path, extension)?.count())
    }

    pub fn file_paths_in_dir(
        path: impl AsRef<Path>,
        extension: Option<&str>,
    ) -> Result<Vec<PathBuf>> {
        let rs_files = files_in_dir(path, extension)?
            .map(|entry| entry.path())
            .collect::<Vec<_>>();
        Ok(rs_files)
    }

    pub fn files_are_equal(path1: impl AsRef<Path>, path2: impl AsRef<Path>) -> Result<bool> {
        const BUF_SIZE: usize = 8192;
        let mut f1 = File::open(path1)?;
        let mut f2 = File::open(path2)?;
        let mut buf1 = [0u8; BUF_SIZE];
        let mut buf2 = [0u8; BUF_SIZE];

        loop {
            let n1 = f1.read(&mut buf1)?;
            let n2 = f2.read(&mut buf2)?;
            if n1 != n2 {
                return Ok(false); // different lengths
            }
            if n1 == 0 && n2 == 0 {
                break; // both EOF
            }
            if buf1[..n1] != buf2[..n2] {
                return Ok(false); // different bytes
            }
        }
        Ok(true)
    }
}

mod tests {
    use super::*;
    #[internal_test_proc_macro::xtest]
    fn ui_tests() {
        let t = trybuild::TestCases::new();
        t.compile_fail(get_trybuild_path_string(SubDir::Root));
        #[cfg(not(feature = "legacy_path_param"))]
        t.compile_fail(get_trybuild_path_string(SubDir::Standard));
        #[cfg(feature = "legacy_path_param")]
        t.compile_fail(get_trybuild_path_string(SubDir::LegacyPathParam));
        #[cfg(nightly)]
        t.compile_fail(get_trybuild_path_string(SubDir::Nightly));
        #[cfg(stable)]
        {
            // prevent rust rovers runner from triggering nightly
            unsafe {
                std::env::remove_var("RUSTC_BOOTSTRAP");
            }
            t.compile_fail(get_trybuild_path_string(SubDir::Stable));
        }
    }

    #[cfg(not(wasm))]
    #[internal_test_proc_macro::xtest]
    fn ensure_ui_tests_for_nightly_and_stable_are_identical() -> std::io::Result<()> {
        let nightly_dir = get_tests_path_string(SubDir::Nightly);
        let stable_dir = get_tests_path_string(SubDir::Stable);

        assert_eq!(
            utils::count_files_in_dir(&nightly_dir, Some(STDERR_EXTENSION))
                .expect("failed to count nightly files"),
            utils::count_files_in_dir(&stable_dir, Some(STDERR_EXTENSION))
                .expect("failed to count stable files"),
            "nightly and stable ui tests should have the same number of stderr files"
        );

        let nightly_files = utils::file_paths_in_dir(&nightly_dir, Some(RS_EXTENSION))
            .expect("failed to get nightly file paths");
        let stable_files = utils::file_paths_in_dir(&stable_dir, Some(RS_EXTENSION))
            .expect("failed to get stable file paths");
        for (stable_file, nightly_file) in stable_files.iter().zip(nightly_files.iter()) {
            assert_eq!(
                stable_file.file_name(),
                nightly_file.file_name(),
                "nightly and stable *.rs file names should be identical"
            );
            assert!(
                utils::files_are_equal(nightly_file, stable_file)?,
                "nightly and stable *.rs files should be identical"
            );
        }

        Ok(())
    }
}
