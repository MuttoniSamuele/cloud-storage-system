# Cloud Storage System

## Introduction

This project is a simple fullstack web application that lets you upload and download files from "the cloud".

It was made using [TypeScript](https://www.typescriptlang.org/) with [Svelte](https://svelte.dev/) on the frontend and [Rust](https://www.rust-lang.org/) with [Axum](https://crates.io/crates/axum) on the backend. It uses [Postgres](https://www.postgresql.org/) to store user data and [Redis](https://redis.io/) to store session data.

This project was made as an assignment. You can read more about it [here](./project-info/project-info.md).


## Installation

This section will guide you through the installation process to deploy the application on your machine.


### Prerequisites

Before starting with the installation, you need to make sure you have [Node.js](https://nodejs.org), [npm](https://www.npmjs.com/) as well as [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) installed on your machine.

You are also going to need [docker](https://docs.docker.com/get-docker/) and [docker-compose](https://docs.docker.com/compose/install/) to run the databases.

Once you have all the prerequisites installed, you can clone this repository and enter the project folder.

```bash
git clone https://github.com/MuttoniSamuele/cloud-storage-system
cd cloud-storage-system
```


### Environment variables

Copy the `.env.example` file and rename it to `.env`.

```bash
cp .env.example .env
```

You can change the values of the variables in this file to suit your needs.


### Installation

Now you can start the installation process.

First, start the databases with docker.

```bash
docker-compose up -d
```

Docker doesn't automatically load the database schema. You can do this by executing the following script.

```bash
./load-schema.sh
```

Next, run the following command to build the backend.

```bash
cargo build --release
```

While the code is compiling, you can build the frontend.

```bash
cd public
npm run build
```

This will create a `public/dist` folder with the static files of the frontend. You can choose to only keep this folder and delete the rest of the frontend files as they will no longer be needed.

After the compilation is done, you can go back to the root of the project and run the server.

To run the application, execute the following command.

```bash
cargo run --release
```

Alternatively, you can run the executable in the `target/release/` folder.
