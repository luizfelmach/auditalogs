use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Operator {
    EqString(String),
    NeqString(String),
    Contains(String),
    StartsWith(String),
    EndsWith(String),
    Regex(String),
    EqInt(i64),
    NeqInt(i64),
    GtInt(i64),
    LtInt(i64),
    BetweenInt(i64, i64),
    EqDate(DateTime<Utc>),
    NeqDate(DateTime<Utc>),
    AfterDate(DateTime<Utc>),
    BeforeDate(DateTime<Utc>),
    BetweenDate(DateTime<Utc>, DateTime<Utc>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub field: String,
    pub op: Operator,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryExpr {
    pub and: Option<Vec<Condition>>,
    pub or: Option<Vec<Condition>>,
    pub not: Option<Vec<Condition>>,
}
