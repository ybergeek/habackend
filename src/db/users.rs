use crate::models::user::User;
use crate::schema::users;
extern crate scrypt;
extern crate time;
use time::PreciseTime;
use scrypt::{ScryptParams, scrypt_simple, scrypt_check};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};


#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub hash: &'a str,
}

pub enum UserCreationError {
    DuplicatedEmail,
    DuplicatedUsername,
}

impl From<Error> for UserCreationError {
    fn from(err: Error) -> UserCreationError {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("users_username_key") => return UserCreationError::DuplicatedUsername,
                Some("users_email_key") => return UserCreationError::DuplicatedEmail,
                _ => {}
            }
        }
        panic!("Error creating user: {:?}", err)
    }
}

pub fn create(
    conn: &PgConnection,
    username: &str,
    email: &str,
    password: &str,
) -> Result<User, UserCreationError> {
    // see https://blog.filippo.io/the-scrypt-parameters
    //let hash = &scrypt_simple(password, &ScryptParams::new(14, 8, 1)).expect("hash error");
    let params = ScryptParams::new(14, 8, 1).unwrap();
// Hash the password for storage
    let hash = &scrypt_simple(password, &params)
        .expect("OS RNG should not fail");
    let new_user = &NewUser {
        username,
        email,
        hash,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
        .map_err(Into::into)
}

pub fn login(conn: &PgConnection, email: &str, password: &str) -> Option<User> {
    let start = PreciseTime::now();
    let user = users::table
        .filter(users::email.eq(email))
        .get_result::<User>(conn)
        .map_err(|err| eprintln!("login_user: {}", err))
        .ok()?;

    let password_matches = scrypt_check(password, &user.hash).is_ok();
    let end = PreciseTime::now();

    println!("{} seconds for functions", start.to(end));


    if password_matches {
        Some(user)
    } else {
        eprintln!(
            "login attempt for '{}' failed: password doesn't match",
            email
        );
        None
    }
}

pub fn find(conn: &PgConnection, id: i32) -> Option<User> {
    users::table
        .find(id)
        .get_result(conn)
        .map_err(|err| println!("find_user: {}", err))
        .ok()
}

// TODO: remove clone when diesel will allow skipping fields
#[derive(AsChangeset, Default, Clone, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UpdateUserData {
    username: Option<String>,
    email: Option<String>,
    bio: Option<String>,
    image: Option<String>,
    // hack to skip the field
    #[column_name = "hash"]
    password: Option<String>,
}

pub fn update(conn: &PgConnection, id: i32, data: &UpdateUserData) -> Option<User> {
    let data = &UpdateUserData {
        password: None,
        ..data.clone()
    };
    diesel::update(users::table.find(id))
        .set(data)
        .get_result(conn)
        .ok()
}
#[derive(AsChangeset)]
#[table_name = "users"]
pub struct ChangePwd<'a> {
    pub hash: &'a str,
}
pub fn update_pwd(conn: &PgConnection, id: i32, password: &str) -> Option<User> {
   // let token = &scrypt_simple(password, &ScryptParams::new(14, 8, 1)).expect("hash error");
    let params = ScryptParams::new(14, 8, 1).unwrap();
// Hash the password for storage
    let token = &scrypt_simple(password, &params)
        .expect("OS RNG should not fail");
    diesel::update(users::table.find(id))
        .set(users::hash.eq(token))
        .get_result::<User>(conn)
        .ok()
}
