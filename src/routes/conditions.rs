use crate::auth::Auth;
use crate::db;
use db::conditions::{NewConditions};
use rocket_contrib::json::{Json,JsonError};



#[post("/conditions", format = "application/json", data = "<new_conditions>")]
pub fn conditions(_auth: Auth, new_conditions: Json<NewConditions>,conn: db::Conn) -> Result<Json<NewConditions>,JsonError<'static>> {
    let insert = NewConditions { ..new_conditions.into_inner() };
    Ok(Json(db::conditions::insert(insert, &conn)))
}
