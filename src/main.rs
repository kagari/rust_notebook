#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate json;

use rocket::response::content::Html;
use rocket_contrib::json::{Json, JsonValue};

#[post("/hello")]
fn hello_test() -> JsonValue {
    json!(
        {
            "code": 200,
            "success": true,
            "payload": {
                "features": [
                    "awesome",
                    "easyAPI",
                    "lowLearningCurve"
                ]
            }
        })
}

#[get("/")]
fn index() -> Html<&'static str> {
    use std::path::Path;
    
    // let path = Path::new("./");
    // println!("files: {:?}", path.file_stem().unwrap());
    let template = "<html><body><h1>Welcome to Rust Notebook!</h1></body></html>";

    Html(template)
}

fn main() {
    rocket::ignite()
        .mount("/api", routes![hello_test])
        .mount("/", routes![index])
        .launch();
}
