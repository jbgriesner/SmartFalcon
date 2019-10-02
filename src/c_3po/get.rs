use std::io;
use rocket::response::NamedFile;

/// Just get he index.html
#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}
