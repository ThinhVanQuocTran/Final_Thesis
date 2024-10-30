mod stats;
mod table_schemas;
mod utils;

use datafusion::common::Result;
use datafusion::execution::context::{SessionConfig, SessionContext};
use stats::Stats;
use utils::read_file;

#[tokio::main]
async fn main() -> Result<()> {
    let sql_query = "
    select p.*
    from people p
    where first_name like 'Jiri' and last_name = 'Prochazka' or pid = 3;";

    // Set up the config
    let config = SessionConfig::default();
    let ctx = SessionContext::new_with_config(config);

    // Create tables in the context
    ctx.sql("CREATE TABLE department (did INT PRIMARY KEY, name VARCHAR(50), year_started INT, year_ended INT)").await?.collect().await?;
    ctx.sql("CREATE TABLE people (pid INT PRIMARY KEY, first_name VARCHAR(50), last_name VARCHAR(50), did INT NOT NULL)").await?.collect().await?;

    // Read the SQL query from a file
    let contents = read_file("query.txt");
    println!("With text:\n{contents}");

    // Parse and optimize the logical plan
    let logical_plan_from_file = ctx.sql(&contents).await?.into_optimized_plan()?;

    // Initialize Stats and process the logical plan
    let mut stat = Stats::new(contents, logical_plan_from_file);
    println!(r#"Optimized Logical Plan:\n"{}""#, stat.logical_plan.display_indent());

    // Process the query and print statistics
    stat.process();
    stat.print_stats();

    Ok(())
}
