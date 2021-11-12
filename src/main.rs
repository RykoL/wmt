extern crate wmt;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let mut conn = Connection::open_in_memory()?;
    conn = wmt::migrate::migrate(conn)?;

    let task = wmt::task::start_task(conn, String::from("Hello"), String::from("Use me"))?;

    println!("Task successfully started: {}", task.description);

    Ok(())
}
