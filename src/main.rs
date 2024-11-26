//!
//! ## Overview
//! Template to have something to get-go in some situations
//!
//! This template provides:
//! - Rocket server
//! - Templates
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
