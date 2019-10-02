use std::path::{Path,PathBuf};
use rocket::response::NamedFile;

/// Just get static_files
#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
