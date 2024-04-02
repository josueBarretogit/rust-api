use axum::{routing::get, Router};
use super::CommonRoutes;

use crate::{controllers::{book_controller, customer_controller, Controller}, models::book_models::Books, repositories::{book_repository::{self, BooksRepository}, customer_repository::CustomerRepository, Repository}};


pub struct CustomerRoutes;










