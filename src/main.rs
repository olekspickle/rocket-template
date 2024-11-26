//!
//! ## Overview
//! Template to have something to get-go in some situations
//!
//! This template provides:
//! - Rocket server
//! - Templates
//!
//! FIXME: for some reason resulting docker image outputs "Hello, world!" and exits.
//! - containerization
//!

use rocket::{
    build, catchers,
    fs::{relative, FileServer},
    launch, routes,
};
use rocket_dyn_templates::Template;

mod handlers;

#[cfg(test)]
mod tests;

#[launch]
fn rocket() -> _ {
    build()
        .mount("/", routes![handlers::index])
        .mount("/first", routes![handlers::first])
        .mount("/second", routes![handlers::second])
        .mount("/third", routes![handlers::third])
        .mount("/", FileServer::from(relative!("static/assets")))
        .register("/", catchers![handlers::not_found])
        .attach(Template::fairing())
}
