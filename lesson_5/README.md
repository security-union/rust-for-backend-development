# Lesson 4: Add a PostgreSQL Database 🚀🔥

## ssh to redis docker
```
 docker exec -it redis_cache /bin/ash 
 redis-cli
```

## Get all keys in redis

```
ZRANGE movies_ranking 0 -1 'WITHSCORES'
```
