#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/basic_compile.rs");
    t.pass("tests/test_generation.rs");
    t.compile_fail("tests/error_struct.rs");
    t.compile_fail("tests/double_auto_error.rs");
}
