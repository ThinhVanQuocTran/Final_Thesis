mod stats;
mod table_schemas;
mod utils;

use datafusion::common::Result;
use datafusion::execution::context::{SessionConfig, SessionContext};
use stats::Stats;
use utils::read_file;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up the config
    let config = SessionConfig::default();
    let ctx = SessionContext::new_with_config(config);

    // Specify the path to your SQL commands file
    let sql_file_path = "commands.sql"; // Adjust the filename as needed

    // Read the SQL commands from the file
    let sql_commands = read_file(sql_file_path);
    
    // Split the commands by new line (assuming each command is on a new line)
    let commands: Vec<&str> = sql_commands.lines().collect();

    // Loop through each command and execute
    for command in commands {
        let command = command.trim();
        if !command.is_empty() {
            match ctx.sql(command).await {
                Ok(_) => println!("Executed: {}", command),
                Err(err) => eprintln!("Error executing command: {}", err),
            }
        }
    }

    // Example SQL query for analysis
    let example_query = "
    SELECT p.*
    FROM people p
    WHERE first_name LIKE 'Jiri' AND last_name = 'Prochazka' OR pid = 3;";

    // Parse and optimize the logical plan
    let logical_plan_from_file = ctx.sql(example_query).await?.into_optimized_plan()?;

    // Initialize Stats and process the logical plan
    let mut stat = Stats::new(example_query.to_string(), logical_plan_from_file);
    println!(r#"Optimized Logical Plan:\n"{}""#, stat.logical_plan.display_indent());

    // Process the query and print statistics
    stat.process();
    stat.print_stats();

    Ok(())
}
