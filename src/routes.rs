use axum::Router;

use crate::{
    models::book_models::{Books, Roles},
    repositories::{customer_repository::CustomerRepository, Repository},
    BooksCreateDTO, BooksRepository, BooksUpdateDto, RolesCreateDTO, RolesUpdateDTO,
};

pub mod book_routes;
pub mod customer_routes;

pub trait CommonRoutes {
    fn set_routes(&self) -> Router;
}

#[derive(Clone, Debug)]
pub struct AppStateBooks<T: Repository<Books, BooksCreateDTO, BooksUpdateDto>> {
    pub repository: T,
}

#[derive(Clone, Debug)]
pub struct AppStateRoles<T: Repository<Roles, RolesCreateDTO, RolesUpdateDTO>> {
    pub repository: T,
}

#[derive(Clone)]
pub struct AppStateCustomer {
    pub repository: CustomerRepository,
}
