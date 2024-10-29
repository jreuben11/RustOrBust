use datafusion::arrow::array::{Int32Array, StringArray};
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::common::DataFusionError;
use datafusion::functions_aggregate::expr_fn::{max, min};
use datafusion::prelude::*;
use std::sync::Arc;

async fn sql_over_csv() -> datafusion::error::Result<()> {
    // register the table
    let ctx = SessionContext::new();
    ctx.register_csv("example", "data/example.csv", CsvReadOptions::new())
        .await?;
    // create a plan to run a SQL query
    let df = ctx
        .sql("SELECT a, MIN(b) FROM example WHERE a <= b GROUP BY a LIMIT 100")
        .await?;
    // execute and print results
    df.show().await?;
    Ok(())
}

async fn df_over_csv() -> datafusion::error::Result<()> {
    // create the dataframe
    let ctx = SessionContext::new();
    let df = ctx
        .read_csv("data/example.csv", CsvReadOptions::new())
        .await?;

    let df = df
        .filter(col("a").lt_eq(col("b")))?
        .aggregate(vec![col("a")], vec![min(col("b"))])?
        .limit(0, Some(100))?;

    // execute and print results
    df.show().await?;
    Ok(())
}

async fn df_in_memory() -> datafusion::error::Result<()> {
    // define a schema.
    let schema = Arc::new(Schema::new(vec![
        Field::new("a", DataType::Utf8, false),
        Field::new("b", DataType::Int32, false),
    ]));

    // define data.
    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(StringArray::from(vec!["a", "b", "c", "d"])),
            Arc::new(Int32Array::from(vec![1, 10, 10, 100])),
        ],
    )?;

    // declare a new context. In spark API, this corresponds to a new spark SQLsession
    let ctx = SessionContext::new();

    // declare a table in memory. In spark API, this corresponds to createDataFrame(...).
    ctx.register_batch("t", batch)?;
    let df = ctx.table("t").await?;

    // construct an expression corresponding to "SELECT a, b FROM t WHERE b = 10" in SQL
    let filter = col("b").eq(lit(10));

    let df = df.select_columns(&["a", "b"])?.filter(filter)?;

    // print the results
    df.show().await?;

    Ok(())
}

fn build_record_batch() -> datafusion::error::Result<RecordBatch> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("letter", DataType::Utf8, false),
        Field::new("number", DataType::Int32, false),
    ]));

    // define data.
    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(StringArray::from(vec!["a", "b", "c"])),
            Arc::new(Int32Array::from(vec![1, 4, 3])),
        ],
    )?;
    Ok(batch)
}

// df = spark.createDataFrame([(1, 4, 3)], ['a', 'b', 'c'])
// df.select(greatest(df.a, df.b, df.c).alias("greatest")).collect()

async fn df_greatest(batch: &RecordBatch, col_name: &str) -> datafusion::error::Result<DataFrame> {
    // declare a new context. In spark API, this corresponds to a new spark SQLsession
    let ctx = SessionContext::new();

    // declare a table in memory. In spark API, this corresponds to createDataFrame(...).
    ctx.register_batch("greatest", batch.clone())?;
    let df = ctx.table("greatest").await?;

    // use max aggregate function on passed in col name
    let df = df.aggregate(vec![], vec![max(col(col_name))])?;

    // Turbofish for propogating async results
    Ok::<DataFrame, DataFusionError>(df)
}

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    sql_over_csv().await?;
    df_over_csv().await?;
    df_in_memory().await?;

    let batch = build_record_batch().unwrap();
    let df1 = df_greatest(&batch, "number").await?;
    df1.show().await?;

    let df2 = df_greatest(&batch, "letter").await?;
    df2.show().await?;

    match df_greatest(&batch, "blah").await {
        Ok(_df) => println!("ok!"),
        Err(e) => {
            // TODO: custom logic for different error types
            match e {
                DataFusionError::ArrowError(_, _) => {}
                DataFusionError::IoError(_) => {}
                DataFusionError::SQL(_, _) => {}
                DataFusionError::NotImplemented(_) => {}
                DataFusionError::Internal(_) => {}
                DataFusionError::Plan(_) => {}
                DataFusionError::Configuration(_) => {}
                DataFusionError::SchemaError(_, _) => {}
                DataFusionError::Execution(_) => {}
                DataFusionError::ExecutionJoin(_) => {}
                DataFusionError::ResourcesExhausted(_) => {}
                DataFusionError::External(_) => {}
                DataFusionError::Context(_, _) => {}
                DataFusionError::Substrait(_) => {}
                // note: 2 new enum vals not in rustdoc:
                DataFusionError::ParquetError(_) => {}
                DataFusionError::ObjectStore(_) => {}
            };
            println!("{}", e.message())
        }
    };
    println!("no panic");
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use datafusion::assert_batches_eq;

    #[tokio::test]
    async fn test_df_greatest() {
        // arrange:
        let batch = build_record_batch().unwrap();

        //act:
        let df1 = df_greatest(&batch, "number").await.unwrap();
        let df2 = df_greatest(&batch, "letter").await.unwrap();

        // assert:
        let count = df1.clone().count().await.unwrap();
        assert!(count == 1); // row count AFAIK

        let batches1 = df1.collect().await.unwrap();
        assert_batches_eq!(
            vec![
                "+----------------------+",
                "| max(greatest.number) |",
                "+----------------------+",
                "| 4                    |",
                "+----------------------+",
            ],
            &batches1
        );

        let batches2 = df2.collect().await.unwrap();
        assert_batches_eq!(
            vec![
                "+----------------------+",
                "| max(greatest.letter) |",
                "+----------------------+",
                "| c                    |",
                "+----------------------+",
            ],
            &batches2
        );

        let is_schema_error = match df_greatest(&batch, "blah").await {
            Ok(_) => false,
            Err(e) => {
                match e {
                    DataFusionError::SchemaError(_, _) => true,
                    _ => false,
                }
            }
        };
        assert!(is_schema_error == true);
    }
}
