#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use rocket::http::RawStr;
use rocket::{Request, Rocket, State};

use lib::db;
use lib::db::get_pool;
use lib::model::{Movie, Storage};

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

#[post("/", data="<movie>")]
fn create_movie(movie: Json<Movie>, state: State<Storage>) -> Json<Option<Movie>> {
    let mut db = state.database.get().unwrap();
    Json(db::insert_movie(movie.0, &mut db))
}

#[delete("/<title>")]
fn delete_movie(title: &RawStr, state: State<Storage>) -> Json<bool> {
    let mut db = state.database.get().unwrap();
    Json(db::delete_movie(title.url_decode().expect("Failed to decode title."), &mut db))
}

fn rocket() -> rocket::Rocket {
    let database = get_pool();
    let storage = Storage { database };
    rocket::ignite().mount(
        "/movies",
        routes![get_movies, get_movie, create_movie, delete_movie],
    ).manage(storage)
}