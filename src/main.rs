mod repository;
mod task;
mod technical_debt;

use repository::{insert_tech_debt, migrate};
use rusqlite::Result;

fn main() -> Result<()> {
    let conn = migrate()?;
    insert_tech_debt(conn, String::from("stuff"))?;

    Ok(())
}
