use dotenv::dotenv;
use oracle::{Connection, Error};
use std::env;

pub fn establish_connection() -> Result<Connection, Error> {
    dotenv().ok();

    let user = env::var("ORACLE_USER").expect("ORACLE_USER must be set");
    let password = env::var("ORACLE_PASSWORD").expect("ORACLE_PASSWORD must be set");
    let host = env::var("ORACLE_HOST").expect("ORACLE_HOST must be set");
    let port = env::var("ORACLE_PORT").expect("ORACLE_PORT must be set");
    let service = env::var("ORACLE_SERVICE").expect("ORACLE_SERVICE must be set");

    let conn_string = format!("//{}:{}/{}", host, port, service);
    let conn = Connection::connect(user, password, conn_string)?;
    Ok(conn)
}
