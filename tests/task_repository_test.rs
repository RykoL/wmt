extern crate wmt;

use rusqlite::{Connection, Result};

#[test]
fn test_start_tast_should_create_a_task_without_finished_time() -> Result<()> {
    let task = Connection::open_in_memory()
        .and_then(wmt::migrate::migrate)
        .and_then(|conn| {
            wmt::task::start_task(
                conn,
                String::from("dependencies"),
                String::from("Upgrading dependencies"),
            )
        })?;

    assert_eq!(task.tech_debt, String::from("dependencies"));
    assert_eq!(task.description, String::from("Upgrading dependencies"));

    Ok(())
}
