#[macro_use] extern crate rocket;

use std::collections::HashMap;
use std::ptr::hash;
use rocket::response::content;
use rocket::response::content::RawHtml;
use rocket_dyn_templates::{context, handlebars, Template};

#[get("/")]
fn index() -> Template {
    let map = vec!["Feet", "Poki"];

    Template::render("homepage", context! { people: map})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
}

