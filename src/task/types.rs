use std::time::Duration;

pub struct FinishedTask {
    pub description: String,
    pub time_spent: Duration,
}

pub struct StartedTask {
    pub tech_debt: String,
    pub description: String,
    pub started_at: Duration,
}
