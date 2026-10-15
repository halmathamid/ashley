use crate::value::{push_param, quote_ident, Order, Query, Value};

/// Start a `SELECT` against `table`.
pub fn select(table: impl Into<String>) -> Select {
    Select {
        table: table.into(),
        columns: Vec::new(),
        filters: Vec::new(),
        order_by: Vec::new(),
        limit: None,
        offset: None,
    }
}

/// Fluent `SELECT` builder.
#[derive(Debug, Clone)]
pub struct Select {
    table: String,
    columns: Vec<String>,
    filters: Vec<(String, Value)>,
    order_by: Vec<(String, Order)>,
    limit: Option<u64>,
    offset: Option<u64>,
}

impl Select {
    /// Restrict the projection. When empty, emits `SELECT *`.
    pub fn columns<I, S>(mut self, cols: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.columns = cols.into_iter().map(Into::into).collect();
        self
    }

    /// Add `column = $n` to the `WHERE` clause.
    pub fn filter(mut self, column: impl Into<String>, value: impl Into<Value>) -> Self {
        self.filters.push((column.into(), value.into()));
        self
    }

    /// Order ascending by `column`.
    pub fn order_by(self, column: impl Into<String>) -> Self {
        self.order_by_dir(column, Order::Asc)
    }

    /// Order by `column` with an explicit direction.
    pub fn order_by_dir(mut self, column: impl Into<String>, dir: Order) -> Self {
        self.order_by.push((column.into(), dir));
        self
    }

    pub fn limit(mut self, n: u64) -> Self {
        self.limit = Some(n);
        self
    }

    pub fn offset(mut self, n: u64) -> Self {
        self.offset = Some(n);
        self
    }

    /// Compile into SQL + parameters.
    pub fn build(self) -> Query {
        let mut params = Vec::new();
        let projection = if self.columns.is_empty() {
            "*".to_string()
        } else {
            self.columns
                .iter()
                .map(|c| quote_ident(c).to_string())
                .collect::<Vec<_>>()
                .join(", ")
        };

        let mut sql = format!(
            "SELECT {} FROM {}",
            projection,
            quote_ident(&self.table)
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

        if !self.order_by.is_empty() {
            let clauses: Vec<String> = self
                .order_by
                .iter()
                .map(|(col, dir)| format!("{} {}", quote_ident(col), dir.as_sql()))
                .collect();
            sql.push_str(" ORDER BY ");
            sql.push_str(&clauses.join(", "));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {limit}"));
        }
        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {offset}"));
        }

        Query { sql, params }
    }
}
