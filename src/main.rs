#[macro_use]
extern crate rocket;

mod models;
mod routes;
mod services;

use models::scheduler::Scheduler;
use rocket::launch;
use services::mongodb::MongoDB;

#[launch]
async fn server() -> _ {
    let db = MongoDB::init().await.unwrap();

    Scheduler::init(&db);

    rocket::build().manage(db).mount("/", routes::routes())
}
