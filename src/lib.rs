//! Ashley — a small, type-safe SQL query builder.
//!
//! Build parameterized `SELECT`, `INSERT`, `UPDATE`, and `DELETE` statements
//! without string concatenation.
//!
//! # Example
//!
//! ```
//! use ashley::{select, Value};
//!
//! let query = select("users")
//!     .columns(["id", "name", "email"])
//!     .filter("active", Value::Bool(true))
//!     .filter("role", Value::Text("admin".into()))
//!     .order_by("name")
//!     .limit(10)
//!     .build();
//!
//! assert_eq!(
//!     query.sql,
//!     "SELECT id, name, email FROM users WHERE active = $1 AND role = $2 ORDER BY name ASC LIMIT 10"
//! );
//! assert_eq!(query.params.len(), 2);
//! ```

mod delete;
mod insert;
mod select;
mod update;
mod value;

pub use delete::{delete, Delete};
pub use insert::{insert, Insert};
pub use select::{select, Select};
pub use update::{update, Update};
pub use value::{Order, Query, Value};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_basic() {
        let q = select("users").columns(["id", "name"]).build();
        assert_eq!(q.sql, "SELECT id, name FROM users");
        assert!(q.params.is_empty());
    }

    #[test]
    fn select_star_when_no_columns() {
        let q = select("users").build();
        assert_eq!(q.sql, "SELECT * FROM users");
    }

    #[test]
    fn select_with_filters_order_limit() {
        let q = select("users")
            .columns(["id", "email"])
            .filter("active", Value::Bool(true))
            .filter("age", Value::Int(18))
            .order_by_dir("created_at", Order::Desc)
            .limit(25)
            .offset(50)
            .build();

        assert_eq!(
            q.sql,
            "SELECT id, email FROM users WHERE active = $1 AND age = $2 ORDER BY created_at DESC LIMIT 25 OFFSET 50"
        );
        assert_eq!(q.params, vec![Value::Bool(true), Value::Int(18)]);
    }

    #[test]
    fn insert_values() {
        let q = insert("users")
            .columns(["name", "email"])
            .values([
                Value::Text("Ada".into()),
                Value::Text("ada@example.com".into()),
            ])
            .returning(["id"])
            .build();

        assert_eq!(
            q.sql,
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id"
        );
        assert_eq!(
            q.params,
            vec![
                Value::Text("Ada".into()),
                Value::Text("ada@example.com".into())
            ]
        );
    }

    #[test]
    fn update_set_and_filter() {
        let q = update("users")
            .set("name", Value::Text("Grace".into()))
            .set("active", Value::Bool(false))
            .filter("id", Value::Int(42))
            .returning(["id", "name"])
            .build();

        assert_eq!(
            q.sql,
            "UPDATE users SET name = $1, active = $2 WHERE id = $3 RETURNING id, name"
        );
        assert_eq!(
            q.params,
            vec![
                Value::Text("Grace".into()),
                Value::Bool(false),
                Value::Int(42)
            ]
        );
    }

    #[test]
    fn delete_with_filter() {
        let q = delete("sessions")
            .filter("user_id", Value::Int(7))
            .filter("expired", Value::Bool(true))
            .build();

        assert_eq!(
            q.sql,
            "DELETE FROM sessions WHERE user_id = $1 AND expired = $2"
        );
        assert_eq!(q.params, vec![Value::Int(7), Value::Bool(true)]);
    }

    #[test]
    fn value_display_helpers() {
        assert_eq!(Value::Null.to_literal(), "NULL");
        assert_eq!(Value::Bool(true).to_literal(), "TRUE");
        assert_eq!(Value::Int(3).to_literal(), "3");
        assert_eq!(Value::Float(1.5).to_literal(), "1.5");
        assert_eq!(Value::Text("a'b".into()).to_literal(), "'a''b'");
    }
}
