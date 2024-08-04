## Description

Given a scale connected to a serial port, provide:

* A Rust script that continuously read and stores the weights on SQLite database.
* A Python script that reads the SQLite database and plots the velocity (derivative of the cumulative weights).

## Usage

Install Rust and Python Poetry, then:

```bash
cargo install sqlx-cli
```

```bash
DATABASE_URL=sqlite://db.sqlite3 sqlx database create
```

```bash
DATABASE_URL=sqlite://db.sqlite3 sqlx migrate run
```

```bash
cargo run
```

```bash
poetry install
```

```bash
poetry run ipython plot.py
```
