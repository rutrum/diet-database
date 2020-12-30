#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate rocket_contrib;
extern crate rocket_cors;

use rocket::http::Status;
use rocket_contrib::json::Json;
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

mod grocery_trip {
    use super::*;
    use diet_database::grocery_trip::*;

    #[get("/grocery_trip")]
    pub fn get_all() -> Json<Vec<GroceryTrip>> {
        let conn = db::create_connection();
        let grocery_trips = db::grocery_trip::select_all(&conn).unwrap_or_default();
        Json(grocery_trips)
    }

    #[post("/grocery_trip", data = "<grocery_trip>")]
    pub fn add(grocery_trip: Json<NewGroceryTrip>) -> Status {
        let grocery_trip = grocery_trip.into_inner();
        let conn = db::create_connection();
        match db::grocery_trip::insert(&conn, grocery_trip) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    }

    #[delete("/grocery_trip", data = "<grocery_trip>")]
    pub fn delete(grocery_trip: Json<GroceryTrip>) -> Status {
        let grocery_trip = grocery_trip.into_inner();
        let conn = db::create_connection();
        match db::grocery_trip::delete(&conn, grocery_trip) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    }
}

fn main() -> Result<(), Error> {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount(
            "/",
            routes![
                bowel::get_all,
                bowel::add,
                bowel::delete,
                store::get_all,
                store::add,
                store::delete,
                grocery_trip::get_all,
                grocery_trip::add,
                grocery_trip::delete,
            ],
        )
        .attach(cors)
        .launch();

    Ok(())
}
