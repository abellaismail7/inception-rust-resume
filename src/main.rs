#[macro_use] extern crate rocket;

use std::path::{PathBuf, Path};
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;

#[get("/<file..>")]
async fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/").join(file);
    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()))
}

#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/index.html");
    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, files])
}
