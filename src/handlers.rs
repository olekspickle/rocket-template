use rocket::{catch, get, Request};
use rocket_dyn_templates::{context, Template};

fn styles() -> String {
    String::from_utf8_lossy(include_bytes!("../static/main.css")).to_string()
}

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "base",
        context! {
            styles: styles(),
            title: "intro",
        },
    )
}

#[get("/")]
pub fn first() -> Template {
    Template::render(
        "first",
        context! {
            styles: styles(),
            title: "first",
        },
    )
}

#[get("/")]
pub fn second() -> Template {
    Template::render(
        "second",
        context! {
            styles: styles(),
            title: "second",
        },
    )
}

#[get("/")]
pub fn third() -> Template {
    Template::render(
        "third",
        context! {
            styles: styles(),
            title: "third",
        },
    )
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        context! {
            styles: styles(),
            title: "Oops",
            uri: req.uri()
        },
    )
}

// Also doable, but not really beneficial
// pub fn customize(tera: &mut Tera) {
//     tera.add_raw_template("about.html", r#"
//         {% extends "tera/base" %}

//         {% block content %}
//             <section id="about">
//               <h1>About - Here's another page!</h1>
//             </section>
//         {% endblock content %}
//     "#).expect("valid Tera template");
// }
