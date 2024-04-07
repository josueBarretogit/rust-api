use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

use crate::{models::customer_model::Customer, repositories::Repository, AppStateCustomer};

use super::Controller;

pub struct CustomerController {}
