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

mod grocery_item {
    use super::*;
    use diet_database::grocery_item::*;

    #[get("/grocery_item")]
    pub fn get_all() -> Json<Vec<GroceryItem>> {
        let conn = db::create_connection();
        let items = db::grocery_item::select_all(&conn).unwrap_or_default();
        Json(items)
    }

    #[post("/grocery_item", data = "<item>")]
    pub fn add(item: Json<NewGroceryItem>) -> Status {
        let item = item.into_inner();
        let conn = db::create_connection();
        match db::grocery_item::insert(&conn, item) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    }

    #[delete("/grocery_item", data = "<item>")]
    pub fn delete(item: Json<GroceryItem>) -> Status {
        let item = item.into_inner();
        let conn = db::create_connection();
        match db::grocery_item::delete(&conn, item) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    }
}

mod weight {
    use super::*;
    use diet_database::weight::*;

    #[get("/weight")]
    pub fn get_all() -> Json<Vec<Weight>> {
        let conn = db::create_connection();
        let items = db::weight::select_all(&conn).unwrap_or_default();
        Json(items)
    }

    #[post("/weight", data = "<item>")]
    pub fn add(item: Json<NewWeight>) -> Status {
        let item = item.into_inner();
        let conn = db::create_connection();
        match db::weight::insert(&conn, item) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    }

    #[delete("/weight", data = "<item>")]
    pub fn delete(item: Json<Weight>) -> Status {
        let item = item.into_inner();
        let conn = db::create_connection();
        match db::weight::delete(&conn, item) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    }
}

mod metric {
    use super::*;
    use diet_database::metric::*;

    #[get("/metric")]
    pub fn get_all() -> Json<Vec<Metric>> {
        let conn = db::create_connection();
        let bowels = db::metric::select_all(&conn).unwrap_or_default();
        Json(bowels)
    }

    #[post("/metric", data = "<bowel>")]
    pub fn add(bowel: Json<NewMetric>) -> Status {
        let bowel = bowel.into_inner();
        let conn = db::create_connection();
        match db::metric::insert(&conn, bowel) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    }

    #[delete("/metric", data = "<bowel>")]
    pub fn delete(bowel: Json<Metric>) -> Status {
        let bowel = bowel.into_inner();
        let conn = db::create_connection();
        match db::metric::delete(&conn, bowel) {
            Ok(_) => Status::Ok,
            Err(_) => Status::InternalServerError,
        }
    }
}

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
                metric::get_all,
                metric::add,
                metric::delete,
                weight::get_all,
                weight::add,
                weight::delete,
                grocery_item::get_all,
                grocery_item::add,
                grocery_item::delete,
            ],
        )
        .attach(cors)
        .launch();

    Ok(())
}
