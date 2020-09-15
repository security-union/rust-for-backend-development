#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use lib::db;
use lib::db::get_pool;
use lib::model::{Movie, Storage};
use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::json::Json;

fn main() {
    rocket().launch();
}

#[get("/")]
fn get_movies(state: State<Storage>) -> Json<Option<Vec<Movie>>> {
    let mut db = state.database.get().unwrap();
    Json(db::read_movies(&mut db).ok())
}

#[get("/<title>")]
fn get_movie(title: &RawStr, state: State<Storage>) -> Json<Option<Movie>> {
    let mut db = state.database.get().unwrap();
    Json(db::read_movie(title.url_decode().expect("Failed to decode title."), &mut db).ok().flatten())
}

#[post("/", data = "<movie>")]
fn create_movie(movie: Json<Movie>, state: State<Storage>) -> Json<Option<Movie>> {
    let mut db = state.database.get().unwrap();
    match db::insert_movie(&movie.0, &mut db).ok() {
        Some(_) => Json(Some(movie.0)),
        None => Json(None)
    }
}

#[delete("/<title>")]
fn delete_movie(title: &RawStr, state: State<Storage>) -> Json<bool> {
    let mut db = state.database.get().unwrap();
    match db::delete_movie(title.url_decode().expect("Failed to decode title."), &mut db).ok() {
        Some(_) => Json(true),
        None => Json(false)
    }
}

fn rocket() -> rocket::Rocket {
    let database = get_pool();
    let storage = Storage { database };
    rocket::ignite().mount(
        "/movies",
        routes![get_movies, get_movie, create_movie, delete_movie],
    ).manage(storage)
}