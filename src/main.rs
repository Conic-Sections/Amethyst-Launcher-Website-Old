/*
 * Hedwig Launcher Website
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

#[macro_use]
extern crate rocket;

use std::path::{Path, PathBuf};

use crate::router::web::assets;
use once_cell::sync::Lazy;
use rocket_dyn_templates::Template;
use router::{
    api::{github_api, github_ci_api},
    web::{download_page, guide, index, update_page, market_page},
};

pub mod controller;
pub mod router;

const DOCUMENT_ROOT: Lazy<PathBuf> = Lazy::new(|| Path::new("document").to_path_buf());

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                download_page,
                update_page,
                market_page,
                github_api,
                github_ci_api,
                assets,
                guide,
            ],
        )
        // .register("/", catchers![])
        .attach(Template::fairing())
}
