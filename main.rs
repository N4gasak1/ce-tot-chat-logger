#[macro_use]
extern crate rocket;
extern crate chrono;

use chrono::prelude::*;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/message?<sender>&<message>")]
fn update(sender: String, message: String) {
    print!("Time: {}, Message: {}, Sender: {}",Local::now().format("%Y-%m-%d %H:%M:%S"), message, sender )
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, update])
}
