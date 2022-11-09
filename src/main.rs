#[macro_use] extern crate rocket;

use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    let map = vec!["Feet", "Poki"];

    Template::render("homepage", context! { people: map})
}

#[post("/button")]
fn button() -> Redirect{
    println!("button pressed");
    Redirect::to("/")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/button", routes![button])
}

