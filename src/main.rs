#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> Template {
    let context = {};
    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/images", FileServer::from(relative!("./images")))
        .attach(Template::fairing())
}
