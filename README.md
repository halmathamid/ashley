# ashley

A small Rust library that **creates parameterized SQL** with a fluent builder API.

Supports `SELECT`, `INSERT`, `UPDATE`, and `DELETE` with PostgreSQL-style `$1`, `$2`, … placeholders.

## Quick start

```bash
cargo test
cargo run --example basic
```

```rust
use ashley::{delete, insert, select, update};

// SELECT
let q = select("users")
    .columns(["id", "name", "email"])
    .filter("active", true)
    .filter("role", "admin")
    .order_by("name")
    .limit(10)
    .build();
// => SELECT id, name, email FROM users WHERE active = $1 AND role = $2 ORDER BY name ASC LIMIT 10

// INSERT
let q = insert("users")
    .columns(["name", "email"])
    .values(["Ada", "ada@example.com"])
    .returning(["id"])
    .build();
// => INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id

// UPDATE
let q = update("users")
    .set("name", "Grace")
    .filter("id", 42_i64)
    .build();
// => UPDATE users SET name = $1 WHERE id = $2

// DELETE
let q = delete("sessions")
    .filter("user_id", 7_i64)
    .build();
// => DELETE FROM sessions WHERE user_id = $1
```

`Query { sql, params }` is ready to hand to drivers such as [sqlx](https://crates.io/crates/sqlx).

## API

| Builder | Entry point | Notes |
|---------|-------------|--------|
| Select  | `select(table)` | filters, `ORDER BY`, `LIMIT` / `OFFSET` |
| Insert  | `insert(table)` | columns + values, optional `RETURNING` |
| Update  | `update(table)` | `SET` + filters, optional `RETURNING` |
| Delete  | `delete(table)` | filters |

Values accept `Value` or types that convert into it (`&str`, `String`, `i64`, `i32`, `bool`, `f64`).
