use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn connection_without_transaction() -> PgConnection {
    dotenv().ok();

    let connection_url = env::var("TEST_DATABASE_URL").expect("No test database url set");

    PgConnection::establish(&connection_url).unwrap()
}

pub fn connection() -> PgConnection {
    let conn = connection_without_transaction();

    conn.begin_test_transaction().unwrap();
    conn
}
