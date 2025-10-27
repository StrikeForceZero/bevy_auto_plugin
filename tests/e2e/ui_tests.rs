use internal_test_util::{
    ui_tests,
    ui_util::UiTest,
};

struct UiTests;

impl UiTest for UiTests {
    fn file_path() -> &'static str {
        file!()
    }
}

ui_tests!(UiTests);
