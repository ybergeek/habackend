use crate::schema::conditions;
use diesel;
use diesel::prelude::*;



#[derive(Queryable,Insertable,Serialize,Deserialize)]
#[table_name = "conditions"]
pub struct NewConditions {
	pub time: chrono::NaiveDateTime,
    pub device_id: String,
    pub temperature: Option<bigdecimal::BigDecimal>,
    pub humidity: Option<bigdecimal::BigDecimal>,
}
pub fn insert(new_conditions: NewConditions , connection: &PgConnection) -> NewConditions {
	diesel::insert_into(conditions::table)
		.values(&new_conditions)
		.get_result(connection).unwrap()

}
