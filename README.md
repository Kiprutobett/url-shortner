# URL Shortener

A full-stack URL shortener built with **Rust, Axum, PostgreSQL, SQLx, and React/Vite**.

The application allows users to create shortened URLs, authenticate with JWTs, manage their URLs, and record click events.

## Tech Stack

### Backend

* Rust
* Axum
* Tokio
* SQLx
* PostgreSQL
* JWT authentication
* Argon2 password hashing

### Frontend

* React
* TypeScript / JavaScript
* Vite

## Features

* User registration
* User login
* JWT authentication
* Password hashing
* Create shortened URLs
* Retrieve user's URLs
* Update URLs
* Delete URLs
* URL click tracking
* PostgreSQL persistence
* Database migrations
* React frontend

## Project Structure

```text
url-shortner/
├── frontend/       # React/Vite frontend
├── migrations/     # PostgreSQL database migrations
├── src/            # Rust backend
│   ├── auth.rs
│   ├── db.rs
│   ├── errors.rs
│   ├── middleware.rs
│   ├── models/
│   └── main.rs
├── tests/           # Backend tests
├── .env.example     # Environment variable template
├── Cargo.toml
└── README.md
```

## Requirements

Before running the project, install:

* Rust
* PostgreSQL
* Node.js and npm

Check your installations:

```bash
rustc --version
cargo --version
psql --version
node --version
npm --version
```

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/Kiprutobett/url-shortner.git
cd url-shortner
```

### 2. Configure the environment

Copy the example environment file:

```bash
cp .env.example .env
```

Edit `.env` and provide your own PostgreSQL credentials and JWT secret.

For example:

```env
DATABASE_URL="postgres://username:password@localhost:5432/urldb"
JWT_SECRET="your-secret-here"
```

**Never commit your `.env` file to GitHub.**

### 3. Create the PostgreSQL database

Create a database named `urldb`:

```bash
createdb urldb
```

Alternatively, create it through PostgreSQL:

```sql
CREATE DATABASE urldb;
```

### 4. Run database migrations

From the project root:

```bash
cargo sqlx migrate run
```

If SQLx CLI is not installed:

```bash
cargo install sqlx-cli
```

Then run:

```bash
cargo sqlx migrate run
```

### 5. Start the backend

```bash
cargo run
```

The backend should start on:

```text
http://localhost:5000
```

### 6. Start the frontend

Open another terminal:

```bash
cd frontend
npm install
npm run dev
```

Vite will display the local frontend address in the terminal.

## Development

Backend:

```bash
cargo run
```

Run tests:

```bash
cargo test
```

Frontend:

```bash
cd frontend
npm run dev
```

## Environment Variables

The application currently uses:

| Variable       | Description                        |
| -------------- | ---------------------------------- |
| `DATABASE_URL` | PostgreSQL connection string       |
| `JWT_SECRET`   | Secret used for JWT authentication |

Keep real credentials in `.env` and never commit them.

## Future Improvements

Possible future improvements include:

* Production deployment
* Custom short URLs
* Advanced analytics
* Rate limiting
* API documentation
* Docker support
* Better frontend UI
* Redis caching
* QR code generation

## License

This project is currently intended for learning and development purposes.
