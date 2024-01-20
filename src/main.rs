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

#[get("/api/<name>")]
fn get_name(name: String) -> Json<Message> {
    let check = Message {
        message: format!("Hello {}", name),
    };

    Json(check)
}

#[get("/hello?<name>&<age>")]
fn hello(name: Option<String>, age: Option<String>) -> String {
    match (name, age) {
        (Some(name), Some(age)) => format!("Hello, {} of age {}!", name, age),
        (_, _) => "Hello!".into(),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_harsh, get_name, hello])
}
