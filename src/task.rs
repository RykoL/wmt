use std::time::Duration;

use rusqlite::{Connection, Result};

use crate::{
    repository::{TechDebt, TechnicalDebtRepository},
    task_repository::TaskRepository,
};

pub enum Task {
    Started(StartedTask),
    Finished(FinishedTask),
}

pub struct FinishedTask {
    pub description: String,
    pub amount: Duration,
}

pub struct StartedTask {
    pub tech_debt: String,
    pub description: String,
    pub started_at: Duration,
}

fn from_db_task(t: crate::task_repository::TaskEntity) -> Task {
    match t.finished {
        Some(finished) => Task::Finished(FinishedTask {
            description: t.description,
            amount: Duration::from_millis(finished),
        }),
        None => Task::Started(StartedTask {
            tech_debt: t.tech_debt.name,
            description: t.description,
            started_at: t.started,
        }),
    }
}

pub fn start_task(conn: Connection, tech_debt_name: String, description: String) -> Result<Task> {
    let tech_debt_repository = TechnicalDebtRepository::new(&conn);
    let task_repository = TaskRepository::new(&conn);

    let debt: TechDebt = match tech_debt_repository.tech_debt_by_name(&tech_debt_name) {
        Some(d) => d,
        None => tech_debt_repository
            .insert_tech_debt(&tech_debt_name)
            .ok()
            .and_then(|_| tech_debt_repository.tech_debt_by_name(&tech_debt_name))
            .unwrap(),
    };

    task_repository
        .create_task(&description, debt.id)
        .and_then(|task_id| task_repository.task_by_id(task_id))
        .map(from_db_task)
}
