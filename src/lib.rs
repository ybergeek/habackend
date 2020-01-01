#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use rocket_cors;

#[macro_use]
extern crate diesel;

use validator;
#[macro_use]
extern crate validator_derive;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

mod auth;
mod config;
mod db;
mod errors;
mod models;
mod routes;
mod schema;

use rocket_contrib::json::JsonValue;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::custom(db::config())
        .mount(
            "/api",
            routes![
                routes::users::post_users,
                routes::users::post_users_login,
                routes::users::put_user,
                routes::users::get_user,
                routes::users::post_users_error,
                routes::users::update_pwd,
                routes::conditions::conditions,
                routes::conditions::query_view,
            ],
        )
        .attach(db::Conn::fairing())
        .attach(rocket_cors::Cors::default())
        .register(catchers![not_found])
}
