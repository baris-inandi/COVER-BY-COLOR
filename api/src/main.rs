#[macro_use]
extern crate rocket;
use std::collections::HashMap;

use rocket::serde::{json::Json, Serialize};
use serde::Deserialize;
use serde_json::Value;

const JSON_DATA: &str = include_str!("../db/albums.json");

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Album {
    name: String,
    artist: String,
    art: String,
    yr: u32,
    discogs: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Albums {
    results: Vec<Album>,
}

#[get("/albums")]
fn albums() -> Json<Albums> {
    let albums: Albums = serde_json::from_str(JSON_DATA).unwrap();
    Json(albums)
}

#[get("/")]
fn index() -> &'static str {
    &JSON_DATA
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, albums])
}
