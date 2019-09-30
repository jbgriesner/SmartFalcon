use crate::c_3po::{static_files,get};

pub fn new() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/", 
            routes![
                static_files::file,
                get::index,
            ],
        )
}
