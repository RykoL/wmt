extern crate wmt;

use rusqlite::{params, Connection};
use wmt::errors::{Error, Result};

#[test]
fn test_start_tast_should_create_a_task_without_finished_time() -> Result<()> {
    let task = Connection::open_in_memory()
        .and_then(wmt::migrate::migrate)
        .map_err(|_| Error::DBError)
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

#[test]
fn test_start_task_returns_error_if_there_is_an_already_opened_task() -> Result<()> {
    let conn = Connection::open_in_memory().and_then(wmt::migrate::migrate)?;

    conn.execute(
        "INSERT INTO tech_debt(name) VALUES(?1)",
        [String::from("dependencies")],
    )?;

    conn.execute(
        "INSERT INTO task(description, tech_debt_id) VALUES(?1, ?2)",
        params![String::from("I don't care"), 1],
    )?;
    let error = wmt::task::start_task(
        &conn,
        String::from("dependencies"),
        String::from("Upgrading dependencies"),
    );

    match error {
        Ok(_) => assert!(false, "Should return an error"),
        Err(err) => assert_eq!(err, Error::AlreadyOpenedTask),
    }
    Ok(())
}
