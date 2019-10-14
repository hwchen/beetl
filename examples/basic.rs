use beetl::MeltRecord;

fn main() {
    simple_handwritten();
    simple_with_manual_trait();
    simple_with_derive();
}

fn simple_handwritten() {
    let rows = get_input_simple();

    // melt rows
    let mut melt = vec![];
    for input_row in rows {
        melt.push(SimpleOutputRow {
            country: input_row.country,
            year: get_year("exports_2016"),
            exports: input_row.exports_2016,
        });
        melt.push(SimpleOutputRow {
            country: input_row.country,
            year: get_year("exports_2017"),
            exports: input_row.exports_2017,
        });
        melt.push(SimpleOutputRow {
            country: input_row.country,
            year: get_year("exports_2018"),
            exports: input_row.exports_2018,
        });
        melt.push(SimpleOutputRow {
            country: input_row.country,
            year: get_year("exports_2019"),
            exports: input_row.exports_2019,
        });
    }

    for output_row in melt {
        println!("{:?}", output_row);
    }
}

fn simple_with_manual_trait() {
    let rows = get_input_simple();

    let melt: Vec<_> = rows.into_iter()
        .flat_map(|input_row| input_row.melt())
        .collect();

    for output_row in melt {
        println!("{:?}", output_row);
    }
}

fn simple_with_derive() {
    let rows = get_input_derived();

    let melt: Vec<_> = rows.into_iter()
        .flat_map(|input_row| input_row.melt())
        .collect();

    for output_row in melt {
        println!("{:?}", output_row);
    }
}

fn get_year(s: &str) -> u32 {
    s.chars().skip(8).collect::<String>().parse().unwrap()
}

#[derive(MeltRecord)]
//#[melt(var(year: u32))]
//#[melt(var_into(get_year))]
//#[melt(value(exports: f32))]
struct DerivedInputRow {
    country: u32,
    #[value_var]
    exports_2016: f32,
    #[value_var]
    exports_2017: f32,
    #[value_var]
    exports_2018: f32,
    #[value_var]
    exports_2019: f32,
}

// ===============================================================================
// template for procedurla macro (custom derive)

struct SimpleInputRow {
    country: u32,
    exports_2016: f32,
    exports_2017: f32,
    exports_2018: f32,
    exports_2019: f32,
}

impl SimpleInputRow {
    fn melt(self) -> SimpleInputRowMelt {
        SimpleInputRowMelt {
            row: self,
            count: 0,
        }
    }
}

struct SimpleInputRowMelt {
    row: SimpleInputRow,
    count: usize,
}

impl Iterator for SimpleInputRowMelt {
    type Item = SimpleInputRowMeltOutput;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 4 {
            let res = SimpleInputRowMeltOutput {
                country: self.row.country,
                year: get_year(
                    match self.count {
                        0 => "exports_2016",
                        1 => "exports_2017",
                        2 => "exports_2018",
                        3 => "exports_2019",
                        _ => return None,
                    }
                ),
                exports: match self.count {
                    0 => self.row.exports_2016,
                    1 => self.row.exports_2017,
                    2 => self.row.exports_2018,
                    3 => self.row.exports_2019,
                    _ => return None,
                },
            };

            self.count += 1;

            Some(res)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct SimpleInputRowMeltOutput {
    country: u32,
    year: u32,
    exports: f32,
}

// ===============================================================================

#[derive(Debug)]
struct SimpleOutputRow {
    country: u32,
    year: u32,
    exports: f32,
}

// helper
fn get_input_simple() -> Vec<SimpleInputRow> {
    vec![
        SimpleInputRow {
            country: 0,
            exports_2016: 100.0,
            exports_2017: 200.0,
            exports_2018: 300.0,
            exports_2019: 400.0,
        },
        SimpleInputRow {
            country: 1,
            exports_2016: 500.0,
            exports_2017: 600.0,
            exports_2018: 700.0,
            exports_2019: 800.0,
        },
    ]
}

// helper
fn get_input_simple() -> Vec<SimpleInputRow> {
fn get_input_derived() -> Vec<DerivedInputRow> {
    vec![
        DerivedInputRow {
            country: 0,
            exports_2016: 100.0,
            exports_2017: 200.0,
            exports_2018: 300.0,
            exports_2019: 400.0,
        },
        DerivedInputRow {
            country: 1,
            exports_2016: 500.0,
            exports_2017: 600.0,
            exports_2018: 700.0,
            exports_2019: 800.0,
        },
    ]
}
