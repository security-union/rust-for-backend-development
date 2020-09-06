# Lesson 3: Dockerize the JSON API 🚀🔥

## Vanilla Docker

1. build docker image
```
docker build -t rust_json_raw_docker -f Dockerfile.raw . 
```

2. run docker container and create and interactive tty session.
```
docker run --rm -it -p 8000:8000 --name rust_json_raw rust_json_raw_docker
```

3. after "connecting" to the container, execute `cargo run`

## Docker + Cargo Watch

1. build docker image
```
docker build -t rust_json_api_local_dev -f Dockerfile.localdevelopment . 
```

2. run docker container and create and interactive tty session.
```
docker run --rm -it -p 8000:8000 \
--mount type=bind,source="$(pwd)",target=/app \
 --name rust_json rust_json_api_local_dev \
 cargo watch -x 'run --bin rest-api'
```

## Docker + Production Image

1. build docker image
```
docker build -t rust_json_api_prod . -f Dockerfile.production
```

2. Push it to your preferred docker image repository

## Makefile

I included some "shortcuts" to make it easier to use, as opposed to typing these very
long commands.

To detach the terminal while running 

```
make local_run
```

Please execute `ctrl+pq`

## Docker image commands

```
docker image ls
```

Remove dangling images

```
docker image prune
```

Remove dangling AND unused images.
```
docker image prune -all
```

## Docker system commands

Remove all unused containers, networks, images (both dangling and unreferenced), and optionally, volumes.
```
docker system prune
```

```
docker system prune --all
```

## Docker containers

```
docker ps -aq
```

Stop all running containers

```
docker stop $(docker ps -aq)
```
Remove all containers

```
docker rm $(docker ps -aq)
```