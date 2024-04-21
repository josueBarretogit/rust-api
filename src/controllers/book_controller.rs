use crate::{controllers::*, AppStateRoles, BooksCreateDTO, RolesRepository};
use axum::{
    debug_handler, extract::State, http::StatusCode, response::IntoResponse, Extension, Json,
};
use serde_json::{json, Value};

use crate::helpers::helpers::Responder;
use crate::models::book_models::{Roles, RolesCreateDTO};
use crate::{models::book_models::Books, repositories::Repository, AppStateBooks, BooksRepository};

pub struct BookController {}

impl super::Controller<Books, AppStateBooks<BooksRepository>, BooksCreateDTO> for BookController {
    async fn handle_get_models(
        state: StateController<AppStateBooks<BooksRepository>>,
    ) -> ApiResponse {
        let repository = &state.repository;

        let response = repository.find_all().await;

        match response {
            Ok(books) => Ok(Responder::Ok(json!(books))),
            Err(database_error) => Err(Responder::DatabaseError(database_error, "books".into())),
        }
    }

    async fn handle_create_model(
        state: StateController<AppStateBooks<BooksRepository>>,
        Json(body): Json<BooksCreateDTO>,
    ) -> ApiResponse {
        let book = Books::default();
        let response: Responder = Responder::Ok(json!(book));
        Ok::<Responder, Responder>(response)
    }
}

pub struct RolesController {}

impl Controller<Roles, AppStateRoles<RolesRepository>, RolesCreateDTO> for RolesController {
    async fn handle_get_models(
        state: StateController<AppStateRoles<RolesRepository>>,
    ) -> ApiResponse {
        let role = Roles::default();
        let response: Responder = Responder::BadRequest("a bad req".to_string());
        Ok::<Responder, Responder>(response)
    }

    async fn handle_create_model(
        state: StateController<AppStateRoles<RolesRepository>>,
        body: Json<RolesCreateDTO>,
    ) -> ApiResponse {
        let response = state.repository.insert(body.0).await;

        match response {
            Ok(res) => Ok(Responder::Ok(json!(res))),
            Err(db_error) => Err(Responder::DatabaseError(db_error, "roles".into())),
        }
    }
}
