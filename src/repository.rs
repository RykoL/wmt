use crate::domain::TechnicalDebt;
use rusqlite::{Connection, Result};

pub fn migrate() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tech_debt (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL
                  )",
        [],
    )?;

    conn.execute(
        "create table if not exists task (
            id INTEGENER PRIMARY KEY,
            description TEXT NOT NULL,
            started INTEGER NOT NULL,
            finished INTEGER,
            tech_debt_id INTEGER NOT NULL,
            FOREIGN KEY(tech_debt_id) REFERENCES tech_debt(id)
        )",
        [],
    )?;

    Ok(conn)
}

pub fn tech_debt_exists(conn: Connection, name: String) -> Option<TechnicalDebt> {
    let debt: Result<TechnicalDebt> =
        conn.query_row("SELECT * FROM tech_debt where name = ?)", [name], |row| {
            Ok(TechnicalDebt {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        });

    match debt {
        Ok(d) => Some(d),
        Err(_) => None,
    }
}

pub fn insert_tech_debt(conn: Connection, name: String) -> Result<()> {
    conn.prepare("INSERT INTO tech_debt (name) VALUES (?1)")?
        .execute([name])?;

    Ok(())
}
