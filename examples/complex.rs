#[derive(MeltRecordWith)]
struct ComplexInputRow {
    country: u32,
    exports_2016: f32,
    exports_2017: f32,
    imports_2016: f32,
    imports_2017: f32,
}

#[derive(Debug)]
struct ComplexOutputRow {
    country: u32,
    flow: String,
    year: u32,
    exports: f32,
}
