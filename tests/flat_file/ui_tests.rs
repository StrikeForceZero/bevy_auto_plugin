#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/flat_file/ui/*.rs");
    #[cfg(not(feature = "legacy_path_param"))]
    t.compile_fail("tests/flat_file/ui/standard/*.rs");
    #[cfg(feature = "legacy_path_param")]
    t.compile_fail("tests/flat_file/ui/legacy_path_param/*.rs");
}
