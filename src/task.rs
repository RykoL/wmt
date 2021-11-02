use std::time::{Duration, Instant};

use crate::repository::{insert_tech_debt, tech_debt_exists};

struct FinishedTask {
    id: i32,
    description: String,
    amount: Duration,
}

struct StartedTask {
    id: i32,
    tech_debt: String,
    started_at: Instant,
}

pub fn start_task(conn: Connection, tech_debt_name: String) -> Result<StartedTask> {
    let debt = match tech_debt_exists(conn, tech_debt_name) {
        Some(d) => d,
        None => insert_tech_debt(conn, tech_debt_name),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_task_returns_unfinished_task() {
        let expected = StartedTask {
            id: 0,
            tech_debt: String::from("foo"),
            started_at: Instant::now(),
        };

        assert_eq!(expected, start_task(String::from("bar")?))
    }
}
