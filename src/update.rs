use crate::value::{push_param, quote_ident, Query, Value};

/// Start an `UPDATE table`.
pub fn update(table: impl Into<String>) -> Update {
    Update {
        table: table.into(),
        sets: Vec::new(),
        filters: Vec::new(),
        returning: Vec::new(),
    }
}

/// Fluent `UPDATE` builder.
#[derive(Debug, Clone)]
pub struct Update {
    table: String,
    sets: Vec<(String, Value)>,
    filters: Vec<(String, Value)>,
    returning: Vec<String>,
}

impl Update {
    /// Add `column = $n` to the `SET` clause.
    pub fn set(mut self, column: impl Into<String>, value: impl Into<Value>) -> Self {
        self.sets.push((column.into(), value.into()));
        self
    }

    /// Add `column = $n` to the `WHERE` clause.
    pub fn filter(mut self, column: impl Into<String>, value: impl Into<Value>) -> Self {
        self.filters.push((column.into(), value.into()));
        self
    }

    pub fn returning<I, S>(mut self, cols: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.returning = cols.into_iter().map(Into::into).collect();
        self
    }

    /// Compile into SQL + parameters.
    ///
    /// # Panics
    /// Panics when no `SET` assignments were provided.
    pub fn build(self) -> Query {
        assert!(!self.sets.is_empty(), "update requires at least one set()");

        let mut params = Vec::new();
        let assignments: Vec<String> = self
            .sets
            .into_iter()
            .map(|(col, val)| {
                let placeholder = push_param(&mut params, val);
                format!("{} = {}", quote_ident(&col), placeholder)
            })
            .collect();

        let mut sql = format!(
            "UPDATE {} SET {}",
            quote_ident(&self.table),
            assignments.join(", ")
        );

        if !self.filters.is_empty() {
            let clauses: Vec<String> = self
                .filters
                .into_iter()
                .map(|(col, val)| {
                    let placeholder = push_param(&mut params, val);
                    format!("{} = {}", quote_ident(&col), placeholder)
                })
                .collect();
            sql.push_str(" WHERE ");
            sql.push_str(&clauses.join(" AND "));
        }

        if !self.returning.is_empty() {
            let ret = self
                .returning
                .iter()
                .map(|c| quote_ident(c).to_string())
                .collect::<Vec<_>>()
                .join(", ");
            sql.push_str(" RETURNING ");
            sql.push_str(&ret);
        }

        Query { sql, params }
    }
}
