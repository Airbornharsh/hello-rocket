use rocket::serde::{json::Json, Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    message: Option<String>,
}

#[get("/")]
fn index() -> &'static str {
    "My First Server using rust"
}

#[get("/api")]
fn get_harsh() -> Json<Message> {
    let check = Message {
        message: Some("Hello Harsh".to_string()),
    };

    Json(check)
}

#[get("/api/<name>")]
fn get_name(name: String) -> Json<Message> {
    let check = Message {
        message: format!("Hello {}", name).into(),
    };

    Json(check)
}

#[get("/hello?<name>&<age>")]
fn hello(name: Option<String>, age: Option<String>) -> Json<Message> {
    let msg = match (name, age) {
        (Some(name), Some(age)) => Message {
            message: format!("Hello {} you are {} years old", name, age).into(),
        },
        (_, _) => Message {
            message: format!("Please provide name and age").into(),
        },
    };

    Json(msg)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_harsh, get_name, hello])
}
