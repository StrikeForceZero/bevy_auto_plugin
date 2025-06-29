#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("/ui/*.rs");
}
