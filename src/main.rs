#[macro_use] extern crate rocket; // dependencies for rocket 

use rocket::serde::json::{Value, json};

#[get("/")] 
fn Hello()-> Value {
    json!("Hello, world!\n") // return a JSON value
}
#[rocket::main] // Class 5 - 8
async fn main() {
    let _ = rocket::build() // build the rocket server
    .mount("/", routes![Hello]) // mount the hello route
    .launch() // launch the rocket server
    .await; // wait for the rocket server to launch
    
}
 