use postgres::error::Error;
use postgres::NoTls;
use postgres::Row;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
/**
 * Copyright [2020] [Dario Alessandro Lencina Talarico]
 * Licensed under the Apache License, Version 2.0 (the "License");
 * y ou may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::env;

use crate::model::Movie;

pub fn get_database_url() -> String {
    env::var("PG_URL").expect("PG_URL must be set")
}

pub fn get_pool() -> Pool<PostgresConnectionManager<NoTls>> {
    let manager = PostgresConnectionManager::new(get_database_url().parse().unwrap(), NoTls);
    let pool_size: u32 = env::var("PG_POOL_SIZE").expect("PG_POOL_SIZE must be set").parse::<u32>().unwrap();
    Pool::builder().max_size(pool_size).build(manager).unwrap()
}

pub fn read_movies(db: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Vec<Movie>, Error> {
    let statement = db
        .prepare(
            "select * from movies",
        )?;

    let movies: Vec<Movie> = db.query(&statement, &[])?
        .iter()
        .map(|row| {
            let title: String = row.get("title");
            let genre: String = row.get("genre");
            Movie {
                title,
                genre,
            }
        }).collect();
    Ok(movies)
}

pub fn read_movie(title: String, db: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Option<Movie>, Error> {
    let statement = db
        .prepare(
            "select * from movies where title = $1 ",
        )?;

    let movie: Option<Movie> = db.query(&statement, &[&title])?
        .iter()
        .fold(None, |_acc, row| {
            let title: String = row.get("title");
            let genre: String = row.get("genre");
            Some(Movie {
                title,
                genre,
            })
        });
    Ok(movie)
}

pub fn insert_movie(movie: &Movie, db: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Vec<Row>, Error> {
    let statement = db
        .prepare(
            "insert into movies (title, genre) values ($1, $2)",
        )?;

    db.query(&statement, &[&movie.title, &movie.genre])
}

pub fn delete_movie(title: String, db: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Vec<Row>, Error> {
    let statement = db
        .prepare(
            "delete from movies where title = $1",
        )?;
    db.query(&statement, &[&title])
}
