use std::i32;

use crate::schema::user_info::dsl::user_info;
use crate::utils::establish_connection;
use diesel::prelude::*;

pub fn delete(del_id: i32) {
    let conn = establish_connection();

    diesel::delete(user_info.find(del_id))
        .execute(&conn)
        .expect("Error deleting user_info");
}
