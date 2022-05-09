# Ray Tracing in Rust

This repository is implementation based on RayTracing in One week, the link below.

https://raytracing.github.io/books/RayTracingInOneWeekend.html

# How to run

This codebase can work on the docker container built with the Dockerfile included.

## Build image

Build image with an arbitrary image name like following.

```console
$ docker build -t raytracing_rust .
```

## Run project

Run the project with cargo on docker container.

```console
$ docker run --rm -it raytracing_rust cargo run
```


