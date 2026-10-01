# AlloRésumé

A dynamic website built in Rusts and Leptos that will hold multiple versions of my résumé.

## Install and run

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, etc. If you run into any trouble, you may need to install one or more of these tools. Please refer to the [ rustup documentation ](https://rustup.rs).

### Tools

Make sure that the Rust toolchains and cargo-leptos are already installed

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
1. `rustup update` - update the rust toolchains to latest
1. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
1. `cargo install cargo-generate` - install cargo-generate binary
1. `cargo install cargo-leptos --locked`
1. `cargo install sqlx-cli` - this installs sqlx utility

### Clone

Clone the repo

`git clone https://bitbucket.org/wallacehub-personal/alloresume.git`

`cd alloresume`

### Database

From the root folder, run the following commands to create the DB and run the initialization sql scripts located in the 'migrations' folder.

`source .env` - Set the environment variables, including DATABASE_URL

`sqlx database setup` - DB create and run the migrations

### Run

You may now build and run the application:

- `cargo leptos build` - Will be the application
- `cargo leptos watch` - Will serve the application and continuously rebuild the application as changes are made
- `cargo leptos serve` - Will build the applicatioin and start the server

## Application access

Once application started, access application from you web browser [ http://localhost:3000 ](http://localhost:3000/)

## Inspiration and Thanks

I learned a lot by examining [ this app ](https://github.com/santhosh7403/realword-app-leptos-axum-sqlite) and by listening to the people in the Leptos Discord server.
