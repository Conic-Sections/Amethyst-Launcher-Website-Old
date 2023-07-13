/*
 * Amethyst Website
 * Copyright (C) 2023 Broken-Deer <old_driver__@outlook.com> and contributors
 *
 * This program is free software, you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::path::{Path, PathBuf};

// use anyhow::Result;
use rocket::{fs::NamedFile, http::Status};
use rocket_dyn_templates::{context, Template};

use crate::controller;

#[get("/assets/<file..>")]
pub async fn assets(file: PathBuf) -> Option<NamedFile> {
    println!("file: {:?}", file);
    NamedFile::open(Path::new("public/assets").to_path_buf().join(file))
        .await
        .ok()
}

#[get("/")]
pub async fn index() -> Template {
    Template::render("index", context! {foo: 123})
}

#[get("/guide/<path..>")]
pub async fn guide(path: PathBuf) -> Result<Template, Status> {
    match controller::guide::main(path).await {
        Ok(t) => Ok(t),
        Err(_) => Err(Status::new(404)),
    }
}

#[get("/download")]
pub async fn download_page() -> Template {
    Template::render("download", context! {})
}

#[get("/update")]
pub async fn update_page() -> Template {
    Template::render("update", context! {})
}
