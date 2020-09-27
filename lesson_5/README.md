# Lesson 5: Add REDIS to your Rocket RUST API 🚀🔥

We use Redis to keep stats of the number of queries for each movie title.

ENPOINT: GET localhost:8000/movies/ranking


## ssh to redis docker
```
 docker exec -it redis_cache /bin/ash 
 redis-cli
```

## Get all keys in redis

```
ZRANGE movies_ranking 0 -1 'WITHSCORES'
```
