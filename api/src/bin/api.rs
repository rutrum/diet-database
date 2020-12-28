#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_cors;
extern crate rocket_contrib;
extern crate chrono;

use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket_cors::{AllowedOrigins, Error};

use api::db;
use diet_database::bowel::*;

#[get("/bowel")]
fn bowel_get() -> Json<Vec<Bowel>> {
    let conn = db::create_connection();
    let bowels = db::get_bowels(&conn).unwrap_or_default();
    Json(bowels)
}

#[post("/bowel", data = "<bowel>")]
fn bowel_add(bowel: Json<NewBowel>) -> Status {
    let bowel = bowel.into_inner();
    let conn = db::create_connection();
    match db::insert_bowel(&conn, bowel) {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

fn main() -> Result<(), Error> {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    }.to_cors()?;

    rocket::ignite()
        .mount("/", routes![bowel_get, bowel_add])
        .attach(cors)
        .launch();

    Ok(())
}
