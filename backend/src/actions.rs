use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;
