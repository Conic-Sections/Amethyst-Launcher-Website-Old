/*
 * Magical Launcher Core
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

use rocket_dyn_templates::Template;
use router::{
    api::github_api,
    web::{assets, download_page, index, update_page},
};

pub mod app;
pub mod router;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![index, assets, download_page, update_page, github_api],
        )
        .attach(Template::fairing())
}
