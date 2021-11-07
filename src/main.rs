mod repository;
mod task;
mod task_repository;
mod technical_debt;

use repository::migrate;
use rusqlite::Result;
use task::start_task;

fn main() -> Result<()> {
    let conn = migrate()?;

    start_task(conn, String::from("Hello"), String::from("Use me"))?;

    Ok(())
}
