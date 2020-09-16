# Lesson 4: Add a PostgreSQL Database 🚀🔥

## Run and attach tty session
```
docker-compose up
```

## Run app in the background
```
docker-compose up -d
```

## Get psql client connected to the library db
```
docker exec -ti lesson_4_postgres_1 psql -U postgres -d library
```

## List movies

```
curl http://localhost:8000/movies
```

## Get a particular movie

```
curl http://localhost:8000/movies/John%20Wick
```

## Insert a movie using curl

```
 curl -d '{"title":"Alien 4", "genre":"Sci-Fi"}' -H "Content-Type: application/json" -X POST http://localhost:8000/movies
```

## Delete a movie using curl

```
curl -X "DELETE" http://localhost:8000/movies/Aliens
```
