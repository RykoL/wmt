extern crate wmt;

use rusqlite::{Connection, Result};

#[test]
fn test_start_tast_should_create_a_task_without_finished_time() -> Result<()> {
    let task = Connection::open_in_memory()
        .and_then(wmt::migrate::migrate)
        .and_then(|conn| {
            wmt::task::start_task(
                &conn,
                String::from("dependencies"),
                String::from("Upgrading dependencies"),
            )
        })?;

    assert_eq!(task.tech_debt, String::from("dependencies"));
    assert_eq!(task.description, String::from("Upgrading dependencies"));

    Ok(())
}

#[test]
fn test_finish_current_task_should_stop_a_started_task() -> Result<()> {
    let conn = Connection::open_in_memory().and_then(wmt::migrate::migrate)?;

    wmt::task::start_task(
        &conn,
        String::from("dependencies"),
        String::from("Upgrading dependencies"),
    )?;

    let started_time: i64 = conn.query_row("select started from task", [], |row| row.get(0))?;

    let finished_task = wmt::task::finish_current_task(&conn)?;

    let finished_time: i64 = conn.query_row("select finished from task", [], |row| row.get(0))?;

    assert_eq!(
        finished_task.time_spent.as_millis(),
        (finished_time - started_time) as u128
    );

    Ok(())
}
