use crate::c_3po::{static_files,get,submit};

use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn new() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/", 
            routes![
                get::index,
                static_files::file,
                submit::submit,
            ],
        )
        .register(catchers![not_found])
}
