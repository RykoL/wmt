use std::time::{Duration, Instant};

use rusqlite::{Connection, Result};

use crate::{
    repository::{TechDebt, TechnicalDebtRepository},
    task_repository::TaskRepository,
};

struct FinishedTask {
    id: i64,
    description: String,
    amount: Duration,
}

struct PausedTask {
    id: i64,
    tech_debt: String,
    description: String,
    started_at: Instant,
    paused_at: Instant,
}

pub struct StartedTask {
    id: i64,
    pub tech_debt: String,
    pub description: String,
    pub started_at: Instant,
}

pub fn start_task(
    conn: Connection,
    tech_debt_name: String,
    description: String,
) -> Result<StartedTask> {
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

    let task = task_repository
        .create_task(&description, debt.id)
        .and_then(|task_id| task_repository.task_by_id(task_id))?;

    println!("{}", task.description);

    Ok(StartedTask {
        id: 0,
        tech_debt: String::from("asdasd"),
        description: String::from("asdasd"),
        started_at: Instant::now(),
    })
}
