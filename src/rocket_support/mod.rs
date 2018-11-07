//! This module includes several functions to help you test your code
//! that uses Rocket. It assumes you have defined a `ROCKET_TEST_DATABASE_URL`
//! environment variable (can be in a .env file if you have enabled the `dotenv`
//! feature).

use diesel::delete;
use diesel::pg::PgConnection;
use diesel::query_builder::IntoUpdateTarget;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::Connection;
use diesel::RunQueryDsl;
use dotenv::dotenv;
use rocket::ignite;
use rocket::local::Client;
use rocket::Route;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::new(manager).expect("could not initiate db pool")
}

/// Returns a connection to the database (specified through the
/// `ROCKET_TEST_DATABASE_URL`) environment variable.
/// The connection will directly hit the database as it is not
/// wrapped in a transaction.
pub fn rocket_connection_without_transaction() -> PgConnection {
    dotenv().ok();

    let connection_url = env::var("ROCKET_TEST_DATABASE_URL").expect("No test database url set");

    PgConnection::establish(&connection_url).unwrap()
}

use diesel::associations::HasTable;
use diesel::pg::Pg;
use diesel::query_builder::{QueryFragment, QueryId};
use diesel::query_source::QuerySource;
use diesel::query_source::Table;

pub fn clean_db<T, U, V, W>(tables: Vec<T>, conn: &PgConnection)
where
    T: IntoUpdateTarget<WhereClause = U> + HasTable<Table = V>,
    U: QueryFragment<Pg> + QueryId,
    V: Table + QueryId + QuerySource<FromClause = W>,
    W: QueryFragment<Pg>,
{
    for table in tables {
        delete(table)
            .execute(conn)
            .expect("Could not clean database");
    }
}

use std::sync::Mutex;
use std::sync::MutexGuard;

lazy_static! {
    static ref G_MUTEX: Mutex<()> = Mutex::new(());
}

// Initializes Rocket. It accepts a Vec<Route> as argument which is the routes that will
// be mounted. It can be generated through the [`routes!`] macro of Rocket.
// It returns a tuple that contain the Rocket client and a mutex.
// Tests that call init_rocket and do no drop this mutex will be run synchronously with
// each other. This allows to avoid database conflicts if your tests are rely on it.
// If you do not want to run a test synchronously, just do not retrieve the mutex.
//
// [`routes`]: https://api.rocket.rs/rocket_codegen/index.html#procedural-macros
pub fn init_rocket_and_clean_tables<T, U, V, W>(
    routes: Vec<Route>,
    tables_to_clean: Vec<T>,
) -> (Client, MutexGuard<'static, ()>)
where
    T: IntoUpdateTarget<WhereClause = U> + HasTable<Table = V>,
    U: QueryFragment<Pg> + QueryId,
    V: Table + QueryId + QuerySource<FromClause = W>,
    W: QueryFragment<Pg>,
{
    let guard = match G_MUTEX.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    dotenv().ok();

    clean_db(tables_to_clean, &rocket_connection_without_transaction());

    let pool = init_pool(&env::var("ROCKET_TEST_DATABASE_URL").expect("No test database url set"));
    let rocket_instance = ignite().manage(pool).mount("/", routes);

    (
        Client::new(rocket_instance).expect("Could not create rocket client"),
        guard,
    )
}

pub fn init_rocket(
    routes: Vec<Route>
) -> (Client, MutexGuard<'static, ()>) {
    let guard = match G_MUTEX.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    dotenv().ok();

    let pool = init_pool(&env::var("ROCKET_TEST_DATABASE_URL").expect("No test database url set"));
    let rocket_instance = ignite().manage(pool).mount("/", routes);

    (
        Client::new(rocket_instance).expect("Could not create rocket client"),
        guard,
    )}
