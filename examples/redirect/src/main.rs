#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

use rocket::response::Redirect;

#[get("/")]
fn root() -> Redirect {
    Redirect::to("/login")
}

#[get("/login")]
fn login() -> &'static str {
    "Hi! Please log in before continuing."
}

fn main() {
    rocket::ignite().mount_and_launch("/", routes![root, login]);
}
