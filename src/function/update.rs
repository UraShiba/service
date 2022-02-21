use crate::schema::user_info;
use crate::utils::establish_connection;
use diesel::prelude::*;

pub fn update_pass(id: i32, pass: String) {
    let conn = establish_connection();
    diesel::update(user_info::dsl::user_info.find(id))
        .set(user_info::pass.eq(pass))
        .execute(&conn)
        .expect("Error updating users");
}
