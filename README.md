# lever
> Flippin' features since 2023

## Background

This was made as a project for Innovation Week at [Teamtailor](https://www.teamtailor.com/), which is a hack week that is every eight week at Teamtailor. And the purpose for this was to try out using Rust since I found it interesting from a lot of perspectives and also more and more companies are adopting it so I wanted to see what the deal was.

Want to know more about how work it is to work as a developer at Teamtailor? [Read a great blog post about it!](https://career.teamtailor.com/posts/building-teamtailor-how-we-work-in-the-product-team)

## Structure

The project consists of three libraries:

* `lever-core`: a library to try out code sharing between the API and also web frontend
* `lever-api`: a crud based api written with [actix](https://actix.rs/) and [diesel](https://diesel.rs/)
* `lever-client`: a web frontend written with [leptos](https://leptos.dev/)

## Prerequisites

Before running `lever-client` you need to have installed both [trunk](https://trunkrs.dev/) and the `wasm32-unknown-unknown` target.

```sh
cargo install trunk
rustup target add wasm32-unknown-unknown
```

## Running

In order to run lever, both the `lever-api` and the `lever-client` has to be run.

Running `lever-api`:
```sh
cd lever-api
cargo run
```

Running `lever-client`:
```sh
cd lever-client
trunk serve --port=8081
```

## Developing

The only thing I can add about how my development journey has been made a lil' bit easier was to use the `cargo-watch` crate for `lever-api` (`lever-client` and `trunk` already has live reloading).

In the `lever-api` folder:
```sh
cargo watch -x run
```

# License

See [license](./LICENSE).