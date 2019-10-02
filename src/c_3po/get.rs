use rocket::response::NamedFile;
use std::io;

/// Just get he index.html
#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}
