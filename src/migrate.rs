use rusqlite::{Connection, Result};

pub fn migrate(conn: Connection) -> Result<Connection> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tech_debt (
                  id              INTEGER PRIMARY KEY AUTOINCREMENT,
                  name            TEXT NOT NULL
                  )",
        [],
    )?;

    conn.execute(
        "create table if not exists task (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            description TEXT NOT NULL,
            started INTEGER(4) NOT NULL DEFAULT (strftime('%s','now')),
            finished INTEGER,
            tech_debt_id INTEGER NOT NULL,
            FOREIGN KEY(tech_debt_id) REFERENCES tech_debt(id)
        )",
        [],
    )?;

    Ok(conn)
}
