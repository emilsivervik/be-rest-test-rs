#[post("/schedule", format = "application/json", data = "<schedule>")]
pub async fn create_schedule<'a>(
    db: &DB,
    schedule: Json<ScheduleJSON>,
) -> Result<Json<ScheduleJSON>, Status> {
    ScheduleAPI::post(&db.schedule_storage, schedule.0)
        .await
        .map(Json)
        .map_err(|err| {
            println!("Internal Error:  {}", err);
            Status::InternalServerError
        })
}

#[get("/schedule", format = "application/json")]
pub async fn list_schedule(db: &DB) -> Result<Json<Vec<ScheduleJSON>>, Status> {
    ScheduleAPI::get_all(&db.schedule_storage)
        .await
        .map(Json)
        .map_err(|err| {
            println!("Internal Error:  {}", err);
            Status::InternalServerError
        })
}

#[get("/schedule/<id>", format = "application/json")]
pub async fn get_schedule(db: &DB, id: &str) -> Result<Json<Option<ScheduleJSON>>, Status> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(Status::BadRequest)
    };
    ScheduleAPI::get(&db.schedule_storage, oid)
        .await
        .map(|v| {
            if v.is_some() {
                Ok(Json(v))
            } else {
                Err(Status::NotFound)
            }
        })
        .map_err(|err| {
            println!("Internal Error:  {}", err);
            Status::InternalServerError
        })?
}

#[put("/schedule/<id>", format = "application/json", data = "<todo>")]
pub async fn update_schedule(
    db: &DB,
    id: &str,
    todo: Json<ScheduleJSON>,
) -> Result<Json<ScheduleJSON>, Status> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(Status::BadRequest)
    };
    ScheduleAPI::put(&db.schedule_storage, oid, todo.0)
        .await
        .map(|v| {
            if let Some(value) = v {
                Ok(Json(value))
            } else {
                Err(Status::NotFound)
            }
        })
        .map_err(|err| {
            println!("Internal Error:  {}", err);
            Status::InternalServerError
        })?
}

#[delete("/schedule/<id>", format = "application/json")]
pub async fn delete_schedule(db: &DB, id: &str) -> Result<Status, Status> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(Status::BadRequest)
    };
    ScheduleAPI::del(&db.schedule_storage, oid)
        .await
        .map(|v| {
            if v {
                return Status::NoContent;
            }
            Status::Ok
        })
        .map_err(|err| {
            println!("Internal Error:  {}", err);
            Status::InternalServerError
        })
}

use crate::models::schedule::{api::ScheduleAPI, ScheduleJSON};
use crate::services::mongodb::DB;
use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json};
