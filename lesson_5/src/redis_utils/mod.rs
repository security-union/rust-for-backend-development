use crate::redis::Commands;
use redis::{RedisError, RedisResult};
use crate::model::RankingEntry;

pub const MOVIES_RANKING: &str = "movies_ranking";

pub fn get_movies_ranking(redis: &mut redis::Connection) -> Result<Vec<RankingEntry>, RedisError> {
    let result: Result<Vec<(String, i64)>, RedisError> = redis
    .zrangebyscore_withscores(
        &MOVIES_RANKING.to_string(),
        0,
        i64::MAX
    );

    return result.map(|vector| {
        vector
            .iter()
            .map(|(title,score)|{
                RankingEntry {
                    title: title.clone(),
                    score: *score
                }
        }).collect()
    })

}

pub fn increment_movie_search_count(redis: &mut redis::Connection, title: &String) -> RedisResult<()> {
    redis.zincr(&MOVIES_RANKING.to_string(), title, 1)
}
