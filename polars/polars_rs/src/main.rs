use std::fs::File;

use chrono::prelude::*;
use polars::prelude::*;

mod quickstart {
    use super::*;

    pub fn basic_read_write() -> Result<DataFrame, Box<dyn std::error::Error>> {
        let mut df: DataFrame = df!(
           "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
            "birthdate" => [
                NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),
                NaiveDate::from_ymd_opt(1985, 2, 15).unwrap(),
                NaiveDate::from_ymd_opt(1983, 3, 22).unwrap(),
                NaiveDate::from_ymd_opt(1981, 4, 30).unwrap(),
            ],
            "weight" => [57.9, 72.5, 53.6, 83.1],  // (kg)
            "height" => [1.56, 1.77, 1.65, 1.75],  // (m)
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

        Ok(df)
    }

    pub fn select(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
        let result = df
            .clone()
            .lazy()
            .select([
                col("name"),
                col("birthdate").dt().year().alias("birth_year"),
                (col("weight") / col("height").pow(2)).alias("bmi"),
            ])
            .collect()?;
        println!("{}", result);

        let result = df
            .clone()
            .lazy()
            .select([
                col("name"),
                (cols(["weight", "height"]) * lit(0.95))
                    //.round(2)
                    .name()
                    .suffix("-5%"),
            ])
            .collect()?;
        println!("{}", result);

        Ok(())
    }

    pub fn with_columns(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
        let result = df
            .clone()
            .lazy()
            .with_columns([
                col("birthdate").dt().year().alias("birth_year"),
                (col("weight") / col("height").pow(2)).alias("bmi"),
            ])
            .collect()?;
        println!("{}", result);
        Ok(())
    }
}

fn main() {
    let df = quickstart::basic_read_write().unwrap();
    let _ = quickstart::select(&df);
    let _ = quickstart::with_columns(&df);
    println!("bye!");
}
