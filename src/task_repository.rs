use std::time::Duration;

use rusqlite::{params, Connection, Result};

use crate::repository::TechDebt;

#[derive(Debug)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub started: Duration,
    pub finished: Duration,
    pub tech_debt: TechDebt,
}

pub struct TaskRepository<'a> {
    conn: &'a Connection,
}

impl TaskRepository<'_> {
    pub fn new(conn: &Connection) -> TaskRepository {
        TaskRepository { conn }
    }

    pub fn create_task(&self, description: &String, tech_debt_id: i64) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO task (description, tech_debt_id) VALUES (?1, ?2)",
            params![description, tech_debt_id],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn task_by_id(&self, task_id: i64) -> Result<Task> {
        self.conn.query_row(
            "SELECT * from task INNER JOIN tech_debt ON tech_debt.id = task.tech_debt_id where task.id = ?1",
            [task_id],
            |row| {
                Ok(Task {
                    id: row.get(0)?,
                    description: row.get(1)?,
                    started: Duration::from_millis(row.get(2)?),
                    finished: Duration::from_millis(row.get(3)?),
                    tech_debt: TechDebt {
                        id: row.get(4)?,
                        name: row.get(5)?,
                    },
                })
            },
        )
    }
}
