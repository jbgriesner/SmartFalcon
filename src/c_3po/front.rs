use crate::c_3po::{static_files,get,submit};
use rocket_contrib::json::JsonValue;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

/// Mount 3 routes on /
/// Attach not_found handler to handle errors
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
