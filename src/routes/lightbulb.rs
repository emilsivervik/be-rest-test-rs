#[post("/lightbulb", format = "application/json", data = "<lightbulb>")]
pub async fn create_lightbulb<'a>(
    storage: &DB,
    lightbulb: Json<LightbulbJSON>,
) -> Result<Json<LightbulbJSON>, Status> {
    LightbulbAPI::post(&storage.lightbulb_storage, lightbulb.0)
        .await
        .map(Json)
        .map_err(|err| {
            println!("Internal Error:  {}", err);
            Status::InternalServerError
        })
}

#[get("/lightbulb", format = "application/json")]
pub async fn get_all_lightbulb(db: &DB) -> Result<Json<Vec<LightbulbJSON>>, Status> {
    LightbulbAPI::get_all(&db.lightbulb_storage)
        .await
        .map(Json)
        .map_err(|err| {
            println!("Internal Error:  {}", err);
            Status::InternalServerError
        })
}

#[get("/lightbulb/<id>", format = "application/json")]
pub async fn get_lightbulb(db: &DB, id: &str) -> Result<Json<Option<LightbulbJSON>>, Status> {
    let Ok(oid) = ObjectId::parse_str(id) else {
         return Err(Status::BadRequest)
     };
    LightbulbAPI::get(&db.lightbulb_storage, oid)
        .await
        .map(|v| {
            if v.is_some() {
                return Ok(Json(v));
            }
            Err(Status::NotFound)
        })
        .map_err(|err| {
            println!("Internal Error:  {}", err);
            Status::InternalServerError
        })?
}

#[put("/lightbulb/<id>", format = "application/json", data = "<todo>")]
pub async fn update_lightbulb(
    db: &DB,
    id: &str,
    todo: Json<LightbulbJSON>,
) -> Result<Json<LightbulbJSON>, Status> {
    let Ok(oid) = ObjectId::parse_str(id) else {
        return Err(Status::BadRequest)
    };
    LightbulbAPI::put(&db.lightbulb_storage, oid, todo.0)
        .await
        .map(|v| {
            if let Some(value) = v {
                return Ok(Json(value));
            }
            Err(Status::NotFound)
        })
        .map_err(|err| {
            println!("Internal Error:  {}", err);
            Status::InternalServerError
        })?
}

#[delete("/lightbulb/<id>", format = "application/json")]
pub async fn delete_lightbulb(db: &DB, id: &str) -> Result<Status, Status> {
    let Ok(oid) = ObjectId::parse_str(id) else {
         return Err(Status::BadRequest)
     };
    LightbulbAPI::del(&db.lightbulb_storage, oid)
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

use crate::models::lightbulb::{api::LightbulbAPI, LightbulbJSON};
use crate::services::mongodb::DB;
use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json};
