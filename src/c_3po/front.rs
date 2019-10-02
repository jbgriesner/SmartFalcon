use crate::c_3po::{static_files,get,submit};

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
}
