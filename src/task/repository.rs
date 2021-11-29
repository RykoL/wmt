use std::time::Duration;

use rusqlite::{params, Connection, OptionalExtension, Result};

use crate::repository::TechDebt;

pub struct TaskEntity {
    pub id: i64,
    pub description: String,
    pub started: Duration,
    pub finished: Option<u64>,
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
        self.conn
            .execute(
                "INSERT INTO task (description, tech_debt_id) VALUES (?1, ?2)",
                params![description, tech_debt_id],
            )
            .map(|_| self.conn.last_insert_rowid())
    }

    pub fn task_by_id(&self, task_id: i64) -> Result<TaskEntity> {
        self.conn.query_row(
            "SELECT * from task INNER JOIN tech_debt ON tech_debt.id = task.tech_debt_id where task.id = ?1",
            [task_id],
            |row| {
                Ok(TaskEntity {
                    id: row.get(0)?,
                    description: row.get(1)?,
                    started: Duration::from_millis(row.get(2)?),
                    finished: row.get(3)?,
                    tech_debt: TechDebt {
                        id: row.get(5)?,
                        name: row.get(6)?,
                    },
                })
            },
        )
    }

    pub fn next_unfinished_task(&self) -> Result<Option<TaskEntity>> {
        self.conn.query_row(
            "select * from task INNER JOIN tech_debt on tech_debt.id = task.tech_debt_id where finished IS NULL order by started asc",
            [],
            |row| {
                Ok(TaskEntity {
                    id: row.get(0)?,
                    description: row.get(1)?,
                    started: Duration::from_millis(row.get(2)?),
                    finished: row.get(3)?,
                    tech_debt: TechDebt {
                        id: row.get(5)?,
                        name: row.get(6)?,
                    },
                })
            },
        ).optional()
    }

    pub fn finish_task(&self, task_id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE task SET finished = (strftime('%s','now')) where id = ?1",
            [task_id],
        )?;

        Ok(())
    }
}
