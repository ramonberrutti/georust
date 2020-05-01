#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::net::IpAddr;
use maxminddb::geoip2;
use rocket_contrib::json::{Json, JsonValue};

#[get("/country/<ip>")]
fn country(ip: IpAddr, reader: rocket::State<maxminddb::Reader<Vec<u8>>>) -> Option<Json<geoip2::model::Country>> {
    reader.lookup(ip).ok().and_then(|country: geoip2::Country| country.country.and_then(|country| Some(Json(country))))
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = std::env::var("DATABASE").or(Err("DATABASE environment not defined."))?;
    let reader = maxminddb::Reader::open_readfile(env).or(Err("Invalid file"))?;
    let error = rocket::ignite()
        .mount("/", routes![country])
        .register(catchers![not_found])
        .manage(reader)
        .launch();
    Err(Box::new(error))
}