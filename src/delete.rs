use crate::value::{push_param, quote_ident, Query, Value};

/// Start a `DELETE FROM table`.
pub fn delete(table: impl Into<String>) -> Delete {
    Delete {
        table: table.into(),
        filters: Vec::new(),
    }
}

/// Fluent `DELETE` builder.
#[derive(Debug, Clone)]
pub struct Delete {
    table: String,
    filters: Vec<(String, Value)>,
}

impl Delete {
    /// Add `column = $n` to the `WHERE` clause.
    pub fn filter(mut self, column: impl Into<String>, value: impl Into<Value>) -> Self {
        self.filters.push((column.into(), value.into()));
        self
    }

    /// Compile into SQL + parameters.
    pub fn build(self) -> Query {
        let mut params = Vec::new();
        let mut sql = format!("DELETE FROM {}", quote_ident(&self.table));

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

        Query { sql, params }
    }
}
