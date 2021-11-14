extern crate wmt;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let mut conn = Connection::open("wmt.db3")?;
    conn = wmt::migrate::migrate(conn)?;

    let task = wmt::task::start_task(&conn, String::from("Hello"), String::from("Use me"))?;
    wmt::task::finish_current_task(&conn)?;

    println!("Task successfully started: {}", task.description);

    Ok(())
}
