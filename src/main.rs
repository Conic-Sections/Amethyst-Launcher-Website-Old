#[macro_use]
extern crate rocket;

use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;
use rocket_dyn_templates::{context, Template};

pub mod app;
pub mod router;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, assets])
        .attach(Template::fairing())
}

#[get("/assets/<file..>")]
pub async fn assets(file: PathBuf) -> Option<NamedFile> {
    println!("file: {:?}", file);
    NamedFile::open(Path::new("public/assets").to_path_buf().join(file)).await.ok()
}

#[get("/")]
pub async fn index() -> Template {
    Template::render("index", context! {foo: 123})
}
