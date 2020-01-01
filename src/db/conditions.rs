use crate::models::conditions::*;
use crate::models::utils::TimeRange;
use crate::schema::conditions;
use diesel;
use diesel::prelude::*;
use diesel::pg::types::sql_types::Timestamptz;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "conditions"]
pub struct NewConditions {
    pub time: chrono::NaiveDateTime,
    pub device_id: String,
    pub temperature: Option<bigdecimal::BigDecimal>,
    pub humidity: Option<bigdecimal::BigDecimal>,
}
pub fn insert(new_conditions: NewConditions, connection: &PgConnection) -> NewConditions {
    diesel::insert_into(conditions::table)
        .values(&new_conditions)
        .get_result(connection)
        .unwrap()
}

pub fn query_view(time_range: TimeRange, db_connection: &PgConnection) -> Vec<ConditionViewJson> {
    use crate::schema::avg_min_max::dsl::*;
    let TimeRange { before, after } = time_range;
    let result = avg_min_max
        .filter(time.lt(before))
        .filter(time.gt(after))
        .order(time.desc())
        .get_results::<ConditionView>(db_connection)
        .expect("Error loading conditions");

    result
        .into_iter()
        .map(|conditionsview| conditionsview.attach())
        .collect()
}

const QUERY: &str = r#"
        select
            a.device_id,
            trunc(avg(temperature), 1) as avg_temp,
           trunc(min(temperature), 1)as min_temp,
           (SELECT TO_CHAR(time, 'yyyy-MM-dd HH24:MI') FROM avg_min_max a2
           WHERE a2.device_id = a.device_id AND a2.temperature = MIN(a.temperature)
            and time > $1 and time < $2
        ORDER BY time DESC LIMIT 1 ) as min_time,
           trunc(max(temperature), 2) as max_temp,
           (SELECT TO_CHAR(time, 'yyyy-MM-dd HH24:MI') FROM avg_min_max a2
           WHERE a2.device_id = a.device_id AND a2.temperature = MAX(a.temperature)
            and time > $1 and time < $2
        ORDER BY time DESC LIMIT 1 ) as max_time
         from avg_min_max a
         where a.time > $1 and a.time < $2
        group by a.device_id;
"#;

pub fn query_avgmaxmin(time_range: TimeRange, db_connection: &PgConnection) -> Vec<AvgMinMaxJson> {
    let TimeRange { before, after } = time_range;
    let avg = FilterAvgMinMax {
        after: after.naive_utc(),
        before: before.naive_utc(),
    };
    let query_result: Vec<AvgMinMax> = diesel::sql_query(QUERY)
        .bind::<Timestamptz, _>(avg.after)
        .bind::<Timestamptz, _>(avg.before)
        .get_results::<AvgMinMax>(db_connection)
        .expect("Error loading conditions");

    query_result
        .into_iter()
        .map(|avgminmax| avgminmax.attach())
        .collect()
}
