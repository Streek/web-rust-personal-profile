#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> Template {
    let context = {};
    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![index])
        .mount("/images", FileServer::from("./images"))
        .attach(Template::fairing())
}
