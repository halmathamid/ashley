/// Bound parameter values used by the query builders.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
}

impl Value {
    /// Render a SQL literal for debugging / offline inspection.
    /// Prefer parameterized queries (`Query::params`) at runtime.
    pub fn to_literal(&self) -> String {
        match self {
            Value::Null => "NULL".into(),
            Value::Bool(v) => {
                if *v {
                    "TRUE".into()
                } else {
                    "FALSE".into()
                }
            }
            Value::Int(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::Text(v) => format!("'{}'", v.replace('\'', "''")),
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Text(value.into())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::Text(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int(value as i64)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

/// Sort direction for `ORDER BY`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Order {
    #[default]
    Asc,
    Desc,
}

impl Order {
    pub(crate) fn as_sql(self) -> &'static str {
        match self {
            Order::Asc => "ASC",
            Order::Desc => "DESC",
        }
    }
}

/// Finished SQL statement plus bound parameters (PostgreSQL `$n` style).
#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub sql: String,
    pub params: Vec<Value>,
}

pub(crate) fn push_param(params: &mut Vec<Value>, value: Value) -> String {
    params.push(value);
    format!("${}", params.len())
}

pub(crate) fn quote_ident(name: &str) -> &str {
    // Identifiers are trusted API inputs; keep them unquoted for readability.
    // Callers must not pass user-controlled identifier strings.
    name
}
