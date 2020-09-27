#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate redis;

use std::env;
use lib::db;
use lib::db::get_pool;
use lib::model::{Movie, Storage, RankingEntry};
use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::json::Json;
use lib::redis_utils::{get_movies_ranking, increment_movie_search_count};

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
    let title_string = title.url_decode().expect("Failed to decode title.");
    let mut redis = state.redis.get_connection().unwrap();

    match increment_movie_search_count(&mut redis, &title_string).err() {
        Some(e) => println!("error {}", e),
        _ => {}
    }
    Json(db::read_movie(title_string, &mut db).ok().flatten())
}

#[post("/", data = "<movie>")]
fn create_movie(movie: Json<Movie>, state: State<Storage>) -> Json<Option<Movie>> {
    let mut db = state.database.get().unwrap();
    let inserted_movie: Option<Movie> = db::insert_movie(&movie.0, &mut db)
        .ok()
        .map(|_| movie.0);
    Json(inserted_movie)
}

#[delete("/<title>")]
fn delete_movie(title: &RawStr, state: State<Storage>) -> Json<bool> {
    let mut db = state.database.get().unwrap();
    let parsed_title = title.url_decode().expect("Failed to decode title.");
    let result: bool = db::delete_movie(parsed_title, &mut db).is_ok();
    Json(result)
}

#[get("/ranking")]
fn most_popular_movies(state: State<Storage>) -> Json<Option<Vec<RankingEntry>>> {
    let mut redis = state.redis.get_connection().unwrap();
    return get_movies_ranking(&mut redis)
        .map_or(Json(None), |movies| Json(Some(movies)));
}

fn rocket() -> rocket::Rocket {
    let database = get_pool();
    let redis = redis::Client::open(env::var("REDIS_URL").expect("REDIS_URL must be set")).unwrap();
    let storage = Storage { database, redis };
    rocket::ignite().mount(
        "/movies",
        routes![get_movies, get_movie, create_movie, delete_movie, most_popular_movies],
    ).manage(storage)
}
