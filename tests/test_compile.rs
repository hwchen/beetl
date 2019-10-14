use beetl::MeltRecord;

#[derive(MeltRecord)]
//#[melt(var(year: u32))]
//#[melt(var_into(get_year))]
//#[melt(value(exports: f32))]
struct SimpleInputRow {
    country: u32,
    partner: u32,
    #[value_var]
    exports_2016: f32,
    #[value_var]
    exports_2017: f32,
    #[value_var]
    exports_2018: f32,
    #[value_var]
    exports_2019: f32,
}

#[derive(MeltRecord)]
struct GenericInputRow<K> where K: Copy {
    country: u32,
    partner: K,
    #[value_var]
    exports_2016: f32,
    #[value_var]
    exports_2017: f32,
    #[value_var]
    exports_2018: f32,
    #[value_var]
    exports_2019: f32,
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
