use std::fmt;

use std::time::Duration;

pub struct FinishedTask {
    pub description: String,
    pub time_spent: Duration,
}

#[derive(Debug, PartialEq)]
pub struct StartedTask {
    pub tech_debt: String,
    pub description: String,
    pub started_at: Duration,
}

impl fmt::Display for StartedTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Project: {}\nDescription: {})",
            self.tech_debt, self.description
        )
    }
}
