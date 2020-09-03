use std::fs;
use crate::model::Movie;

static MOVIES_DB: &str = "data/movies.json";

fn _movies() -> Result<Vec<Movie>, serde_json::Error> {
    let data = fs::read_to_string(MOVIES_DB).expect("Error reading from file");
    let movies: Result<Vec<Movie>, serde_json::Error> = serde_json::from_str(&data);
    movies
}

fn _write_movies(movies: Vec<Movie>) {
    let data = serde_json::to_string(&movies).expect("Failed to turn movies into serde string");
    fs::write(MOVIES_DB, data).expect("Failed to write data.");
}

pub fn read_movies() -> Option<Vec<Movie>> {
    match _movies() {
        Ok(movies) => Some(movies),
        Err(_) => None
    }
}

pub fn read_movie(title: String) -> Option<Movie> {
    match _movies() {
        Ok(movies) => {
            let index = movies.iter().position(|m| m.title == title);

            match index {
                Some(x) => Some(movies[x].clone()),
                None => None,
            }
        },
        Err(_) => None
    }
}

pub fn insert_movie(movie: Movie) -> Option<Movie> {
    match _movies() {
        Ok(mut movies) => {
            movies.push(movie.clone());
            _write_movies(movies);
            Some(movie)
        },
        Err(_) => None
    }
}

pub fn delete_movie(title: String) -> bool {
    match _movies() {
        Ok(mut movies) => {
            let index = movies.iter().position(|m| m.title == title);

            match index {
                Some(x) => {
                    movies.remove(x);
                    _write_movies(movies);
                    true
                },
                None => false,
            }
        },
        Err(_) => false
    }
}