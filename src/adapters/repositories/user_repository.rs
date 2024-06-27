use crate::domain::entities::User;
use oracle::Connection;

pub fn insert_user(conn: &Connection, id: u32, name: &str) -> Result<(), oracle::Error> {
    conn.execute(
        "INSERT INTO users (id, name) VALUES (:id, :name)",
        &[&id, &name],
    )?;
    Ok(())
}

pub fn select_user(conn: &Connection, id: u32) -> Result<Option<User>, oracle::Error> {
    let mut stmt = conn
        .statement("SELECT name FROM users WHERE id = :id")
        .build()?;
    let mut rows = stmt.query(&[&id])?;

    if let Some(row) = rows.next() {
        let name: String = row?.get("name")?;
        Ok(Some(User { id, name }))
    } else {
        Ok(None)
    }
}
