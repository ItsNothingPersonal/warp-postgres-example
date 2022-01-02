# Rust ToDo App

A small todo app written in rust with the help of warp, tokio and some other little helpers

## Description

My slightly modified/updated version from the one presented in this article by [Mario Zupan](https://blog.logrocket.com/author/mariozupan/) from [LogRocket](https://logrocket.com).
I had some minor issues with his version, probably because I tried to replicate it on a more modern version of rust and the dependencies in question, so I went ahead and, as mentioned, updated it.
Also added cargo-husky to run some basic checks once you start committing stuff and itconfig for easier handling of configuration variables.

## Features

- warp + tokio + mobc
- postgres integration
- cargo-husky integration
- itconfig

# Project setup

- clone repository
- make sure you either have postgres installed locally or via docker
- create the database and configure the required environment variables
- run `cargo run` or `RUST_LOG=debug cargo run`  
  

## Acknowledgements

- [Create an async CRUD web service in Rust with warp](https://blog.logrocket.com/create-an-async-crud-web-service-in-rust-with-warp/) by [Mario Zupan](https://blog.logrocket.com/author/mariozupan/)
- [the original repo](https://github.com/zupzup/warp-postgres-example)

## License

[Apache 2.0](https://choosealicense.com/licenses/apache-2.0/)
