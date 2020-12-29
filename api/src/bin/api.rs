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

mod bowel {
    use super::*;
    use diet_database::bowel::*;

    #[get("/bowel")]
    pub fn get_all() -> Json<Vec<Bowel>> {
        let conn = db::create_connection();
        let bowels = db::bowel::select_all(&conn).unwrap_or_default();
        Json(bowels)
    }

    #[post("/bowel", data = "<bowel>")]
    pub fn add(bowel: Json<NewBowel>) -> Status {
        let bowel = bowel.into_inner();
        let conn = db::create_connection();
        match db::bowel::insert(&conn, bowel) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    }

    #[delete("/bowel", data = "<bowel>")]
    pub fn delete(bowel: Json<Bowel>) -> Status {
        let bowel = bowel.into_inner();
        let conn = db::create_connection();
        match db::bowel::delete(&conn, bowel) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    }
}

mod store {
    use super::*;
    use diet_database::store::*;

    #[get("/store")]
    pub fn get_all() -> Json<Vec<Store>> {
        let conn = db::create_connection();
        let stores = db::store::select_all(&conn).unwrap_or_default();
        Json(stores)
    }

    #[post("/store", data = "<store>")]
    pub fn add(store: Json<NewStore>) -> Status {
        let store = store.into_inner();
        let conn = db::create_connection();
        match db::store::insert(&conn, store) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    }

    #[delete("/store", data = "<store>")]
    pub fn delete(store: Json<Store>) -> Status {
        let store = store.into_inner();
        let conn = db::create_connection();
        match db::store::delete(&conn, store) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    }
}

fn main() -> Result<(), Error> {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    }.to_cors()?;

    rocket::ignite()
        .mount("/", routes![
               bowel::get_all,
               bowel::add,
               bowel::delete,
               store::get_all,
               store::add,
               store::delete,
        ])
        .attach(cors)
        .launch();

    Ok(())
}
