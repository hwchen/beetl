use beetl::MeltRecord;

#[derive(MeltRecord)]
struct Record {
    id: u32,
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
