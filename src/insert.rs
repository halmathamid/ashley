use crate::value::{push_param, quote_ident, Query, Value};

/// Start an `INSERT INTO table`.
pub fn insert(table: impl Into<String>) -> Insert {
    Insert {
        table: table.into(),
        columns: Vec::new(),
        values: Vec::new(),
        returning: Vec::new(),
    }
}

/// Fluent `INSERT` builder.
#[derive(Debug, Clone)]
pub struct Insert {
    table: String,
    columns: Vec<String>,
    values: Vec<Value>,
    returning: Vec<String>,
}

impl Insert {
    pub fn columns<I, S>(mut self, cols: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.columns = cols.into_iter().map(Into::into).collect();
        self
    }

    /// Provide one row of values matching [`columns`](Self::columns).
    pub fn values<I, V>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<Value>,
    {
        self.values = vals.into_iter().map(Into::into).collect();
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
    /// Panics when column and value counts differ, or either is empty.
    pub fn build(self) -> Query {
        assert!(
            !self.columns.is_empty(),
            "insert requires at least one column"
        );
        assert_eq!(
            self.columns.len(),
            self.values.len(),
            "insert column count must match value count"
        );

        let mut params = Vec::new();
        let col_list = self
            .columns
            .iter()
            .map(|c| quote_ident(c).to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let placeholders: Vec<String> = self
            .values
            .into_iter()
            .map(|v| push_param(&mut params, v))
            .collect();

        let mut sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            quote_ident(&self.table),
            col_list,
            placeholders.join(", ")
        );

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
