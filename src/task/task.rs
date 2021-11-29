use crate::task::repository::TaskRepository;
use crate::task::types::{FinishedTask, StartedTask};
use std::time::Duration;

use rusqlite::Connection;

use crate::errors::{Error, Result};
use crate::repository::{TechDebt, TechnicalDebtRepository};

pub fn next_open_task(conn: &Connection) -> Option<StartedTask> {
    let task_repository = TaskRepository::new(conn);

    match task_repository.next_unfinished_task() {
        Ok(task) => task.map(|t| StartedTask {
            tech_debt: t.tech_debt.name,
            description: t.description,
            started_at: t.started,
        }),
        Err(_) => None,
    }
}

pub fn start_task(
    conn: &Connection,
    tech_debt_name: String,
    description: String,
) -> Result<StartedTask> {
    let tech_debt_repository = TechnicalDebtRepository::new(conn);
    let task_repository = TaskRepository::new(conn);

    let debt: TechDebt = match tech_debt_repository.tech_debt_by_name(&tech_debt_name) {
        Some(d) => d,
        None => tech_debt_repository
            .insert_tech_debt(&tech_debt_name)
            .ok()
            .and_then(|_| tech_debt_repository.tech_debt_by_name(&tech_debt_name))
            .unwrap(),
    };

    match next_open_task(conn) {
        Some(t) => Err(Error::AlreadyOpenedTask(t)),
        None => Ok(()),
    }?;

    task_repository
        .create_task(&description, debt.id)
        .and_then(|task_id| task_repository.task_by_id(task_id))
        .map(|t| StartedTask {
            tech_debt: t.tech_debt.name,
            description: t.description,
            started_at: t.started,
        })
        .map_err(|err| Error::TaskCreationError)
}

pub fn finish_current_task(conn: &Connection) -> Result<FinishedTask> {
    let task_repository = TaskRepository::new(conn);

    let task = task_repository
        .next_unfinished_task()
        .map_err(|_| Error::MissingTask)?
        .unwrap();

    task_repository
        .finish_task(task.id)
        .and_then(|_| task_repository.task_by_id(task.id))
        .map(|t| FinishedTask {
            description: t.description,
            time_spent: Duration::from_millis(t.finished.unwrap()) - t.started,
        })
        .map_err(|_| Error::MissingTask)
}
