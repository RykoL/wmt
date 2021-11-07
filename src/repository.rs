use rusqlite::{Connection, Result};

pub fn migrate() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;

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

#[derive(Debug)]
pub struct TechDebt {
    pub id: i64,
    pub name: String,
}

pub struct TechnicalDebtRepository<'a> {
    conn: &'a Connection,
}

impl TechnicalDebtRepository<'_> {
    pub fn new(conn: &Connection) -> TechnicalDebtRepository {
        TechnicalDebtRepository { conn }
    }

    pub fn tech_debt_by_name(&self, name: &String) -> Option<TechDebt> {
        let debt: Result<TechDebt> =
            self.conn
                .query_row("SELECT * FROM tech_debt where name = ?", [name], |row| {
                    Ok(TechDebt {
                        id: row.get(0)?,
                        name: row.get(1)?,
                    })
                });

        match debt {
            Ok(d) => Some(d),
            Err(_) => None,
        }
    }

    pub fn insert_tech_debt(&self, name: &String) -> Result<()> {
        self.conn
            .prepare("INSERT INTO tech_debt (name) VALUES (?1)")?
            .execute([name])?;

        Ok(())
    }
}
