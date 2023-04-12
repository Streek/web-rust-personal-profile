#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct App {
    name: String,
    description: String,
    image: String,
    link: String,
}

fn create_app_list() -> Vec<App> {
    vec![
        App {
            name: "App 1".to_string(),
            description: "A description of App 1".to_string(),
            image: "app1.png".to_string(),
            link: "https://example.com/app1".to_string(),
        },
        App {
            name: "App 1".to_string(),
            description: "A description of App 1".to_string(),
            image: "app1.png".to_string(),
            link: "https://example.com/app1".to_string(),
        },
    ]
}

#[get("/")]
fn index() -> Template {
    let context = {};
    Template::render("index", &context)
}

#[get("/apps")]
fn applications() -> Template {
    let context: Vec<App> = create_app_list();
    Template::render("apps", &context)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![index])
        .mount("/apps", routes![applications])
        .mount("/images", FileServer::from("./images"))
        .attach(Template::fairing())
}
