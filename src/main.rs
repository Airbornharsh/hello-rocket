#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "My First Server using rust"
}

#[get("/api")]
fn get_harsh() -> &'static str {
    "Welcome to Rust World"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_harsh])
}
