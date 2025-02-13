use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};

use crate::schema::slides;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, QueryableByName, Insertable)]
#[diesel(table_name = slides)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Slide {
    pub id: String,
    pub caption: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub active: bool,
    pub filetype: String,
}