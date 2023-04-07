## Simple Rust CRUD API with Actix Web and SQLx

This is a simple CRUD API built with Actix Web and SQLx. It is only for learning purposes and is not meant to be used in production.

### Running the API

To run the API, you need to have Rust installed. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

#### Initialize the environment

Before you run the API, you need to create a `.env` file in the root directory of the project from `.env.tpl`.

#### Database

The API uses a PostgreSQL database. You can run the database in a Docker container by running the following command:

```bash
docker compose up -d
```

#### Install the dependencies

To install the dependencies, run the following command:

```bash
cargo install
```

Run `cargo install sqlx-cli` to install the SQLX-CLI if you do not already have it. You can then run the following command to make sure that the database is up-to-date:

```bash
sqlx migrate run
```

#### Run the API

To run the API, run the following command:

```bash
cargo run
```

By default, the API will run on port `8000`.

### API Endpoints

The API has the following endpoints:
* `GET /api/v1/users` - Get all users
* `GET /api/v1/users/{id}` - Get a user by ID
* `POST /api/v1/users` - Create a user
* `GET /health` - Health check
