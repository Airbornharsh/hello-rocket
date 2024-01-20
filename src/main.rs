use rocket::serde::{json::Json, Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "My First Server using rust"
}

#[get("/api")]
fn get_harsh() -> Json<Message> {
    let check = Message {
        message: "Hello Harsh".to_string(),
    };

    Json(check)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_harsh])
}
