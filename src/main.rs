use actix_web::web::Data;
use actix_web::{App, HttpServer};
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use std::io::Result;

mod model;
mod response;
mod schema;
mod user;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("could not read DATABASE_URL");
    let db_connection = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(db_connection)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(user::register)
            .service(user::login)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
