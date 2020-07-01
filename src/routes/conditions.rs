use crate::auth::Auth;
use crate::db;
use crate::models;
use chrono::Local;
use db::conditions::NewConditions;
use models::utils::*;
use rocket_contrib::json::{Json, JsonError, JsonValue};

#[post("/conditions", format = "application/json", data = "<new_conditions>")]
pub fn conditions(
    _auth: Auth,
    new_conditions: Json<NewConditions>,
    conn: db::Conn,
) -> Result<Json<NewConditions>, JsonError<'static>> {
    let insert = NewConditions {
        ..new_conditions.into_inner()
    };
    Ok(Json(db::conditions::insert(insert, &conn)))
}

#[get("/queryview")]
pub fn query_view(_auth: Auth, conn: db::Conn) -> JsonValue {
    let time = Local::now();
    println!("time Local now {}", time);
    let halfhourago =
        db::conditions::query_view(
            halfourago(time), &conn);
    let onedayago =
        db::conditions::query_view(
            oneday(time), &conn);
    let oneyearago =
        db::conditions::query_view(
            oneyear(time), &conn);
    let twoyearago =
        db::conditions::query_view(
            twoyear(time), &conn);
    let threeyearago =
        db::conditions::query_view(
            threeyear(time), &conn);
    let avgminmaxday =
        db::conditions::query_avgmaxmin(
            onedayavg(time), &conn);
    let avgminmaxweek =
        db::conditions::query_avgmaxmin(
            oneweek(time), &conn);
    let avgminmaxmonth =
        db::conditions::query_avgmaxmin(
            onemonth(time), &conn);

    json!({"measurementview": {
            "halfhourago": halfhourago,
            "onedayago": onedayago,
            "oneyearago":oneyearago,
            "twoyearago":twoyearago,
            "threeyearago":threeyearago,
            "avgminmaxday":avgminmaxday,
            "avgminmaxweek":avgminmaxweek,
            "avgminmaxmonth":avgminmaxmonth},
            })
}
