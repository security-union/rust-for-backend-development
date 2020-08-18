#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use rocket::http::RawStr;
use lib::db;
use lib::model::Movie;

fn main() {
    rocket().launch();
}

#[get("/")]
fn get_movies() -> Json<Option<Vec<Movie>>> {
    Json(db::read_movies())
}

#[get("/<title>")]
fn get_movie(title: &RawStr) -> Json<Option<Movie>> {
    Json(db::read_movie(title.url_decode().expect("Failed to decode title.")))
}

#[post("/", data="<movie>")]
fn create_movie(movie: Json<Movie>) -> Json<Option<Movie>> {
    Json(db::insert_movie(movie.0))
}

#[delete("/<title>")]
fn delete_movie(title: &RawStr) -> Json<bool> {
    Json(db::delete_movie(title.url_decode().expect("Failed to decode title.")))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/movies",
        routes![get_movies, get_movie, create_movie, delete_movie],
    )
}