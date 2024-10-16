use std::fs::File;

use chrono::prelude::*;
use polars::prelude::*;

mod quickstart {
    use super::*;

    pub fn basic_read_write() -> Result<(), Box<dyn std::error::Error>> {
        let mut df: DataFrame = df!(
            "integer" => &[1, 2, 3],
            "date" => &[
                    NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                    NaiveDate::from_ymd_opt(2025, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                    NaiveDate::from_ymd_opt(2025, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            ],
            "float" => &[4.0, 5.0, 6.0],
            "string" => &["a", "b", "c"],
        )
        .unwrap();
        println!("{}", df);

        let file_path = "../data/output.csv";
        let mut file = File::create(file_path).expect("could not create file");
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut df)?;

        let df_csv = CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .try_into_reader_with_file_path(Some(file_path.into()))?
            .finish()?;

        println!("{}", df_csv);

        Ok(())
    }
}

fn main() {
    let _ = quickstart::basic_read_write().unwrap();
}
