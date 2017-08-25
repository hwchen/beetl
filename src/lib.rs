#![allow(unused_imports)] // temporary for the macro_use
#![allow(unused_attributes)] // temporary for the macro_use
#![feature(custom_attribute)]

#[macro_use]
extern crate beetl_macros;
pub use beetl_macros::*;

pub trait CreateTablePostgres {
    fn create_table(&self) -> String;
}

pub trait MeltRecord {
    type V;
    type T;

    fn melt<F>(&mut self, out: &mut Vec<Self::T>, f: &F) where  F: Fn(&str, Self::V, &Self, &mut Vec<Self::T>);
}

#[cfg(test)]
mod tests {
    use super::{CreateTablePostgres, MeltRecord};

    #[derive(CreateTablePostgres, Debug)]
    struct TestTable {
        id: u32,
        job: String,
        salary: f32,
        date: String,
    }

    #[test]
    fn test_derive_postgres_table() {
        let test = TestTable {
            id: 1,
            job: "1".to_owned(),
            salary: 1.0,
            date: "1".to_owned(),
        };
        assert_eq!(test.create_table(), "Create Postgres Table".to_owned());
    }

    #[test]
    fn test_melt() {
        #[derive(MeltRecord, Clone, Debug)]
        #[melt_targets(TestRecordOut)]
        struct TestRecord {
            id: usize,
            job: Option<String>,
            #[melt(value_var)]
            //#[melt(rename_val="2001")]
            salary_2001: Option<f32>,
            #[melt(value_var)]
            //#[melt(rename_val="2002")]
            salary_2002: Option<f32>,
            #[melt(value_var)]
            //#[melt(rename_val="2003")]
            salary_2003: Option<f32>,
        };

        #[derive(Debug, PartialEq)]
        struct TestRecordOut {
            id: usize,
            job: Option<String>,
            salary: f32,
            salary_year: String,
        };

        let records = vec![
            TestRecord {
                id: 1,
                job: Some("1".to_owned()),
                salary_2001: Some(11.0),
                salary_2002: Some(21.0),
                salary_2003: Some(31.0),
            },
            TestRecord {
                id: 2,
                job: Some("2".to_owned()),
                salary_2001: Some(7.0),
                salary_2002: Some(8.0),
                salary_2003: Some(9.0),
            },
        ];

        let mut res = Vec::new();
        records[0]
            .clone()
            .melt(&mut res,
                &|var: &str, val: Option<f32>, input: &TestRecord, out: &mut Vec<TestRecordOut>| {
                    if val.is_some() {
                        out.push(TestRecordOut {
                            id: input.id,
                            job: input.job.clone(),
                            salary: val.unwrap(),
                            salary_year: var.to_owned(),
                        });
                    }
                }
            );

        assert_eq!(
            res[0],
            TestRecordOut {
                id: 1,
                job: Some("1".to_owned()),
                salary: 11.0,
                salary_year: "salary_2001".to_owned(),
            }
        );
        assert_eq!(
            res[1],
            TestRecordOut {
                id: 1,
                job: Some("1".to_owned()),
                salary: 21.0,
                salary_year: "salary_2002".to_owned(),
            }
        );
        assert_eq!(
            res[2],
            TestRecordOut {
                id: 1,
                job: Some("1".to_owned()),
                salary: 31.0,
                salary_year: "salary_2003".to_owned(),
            }
        );
    }
}
