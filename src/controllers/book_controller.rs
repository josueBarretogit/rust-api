use axum::{
    debug_handler, extract::State, http::StatusCode, response::IntoResponse, Extension, Json,
};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use crate::{controllers::*, AppStateRoles, RolesRepository};

use crate::helpers::helpers::Responder;
use crate::models::book_models::Roles;
use crate::{models::book_models::Books, repositories::Repository, AppStateBooks, BooksRepository};

pub struct BookController {}

impl super::Controller<Books, AppStateBooks<BooksRepository>> for BookController {

    async fn handle_get_models(state: StateController<AppStateBooks<BooksRepository>>) -> Result<impl IntoResponse, impl IntoResponse> {
        
        let repository = &state.repository;

        let response = repository.find_all().await;

        match response {
            Ok(books) => Ok(Responder::Ok(json!(books))),
            Err(database_error) => Err(Responder::DatabaseError(database_error))  
        }
    }


    async fn handle_create_model(state: StateController<AppStateBooks<BooksRepository>>, Json(body): Json<Books>) -> Result<impl IntoResponse, impl IntoResponse> { 
        let book = Books::new(Some("a".into()), None);
        let response: Responder =   Responder::BadRequest("a bad req".to_string());
            Ok::<Responder, Responder>(response)
    }
}

pub struct  RolesController {}

impl Controller<Roles,  AppStateRoles<RolesRepository>> for RolesController {
    
    async fn handle_get_models( state: StateController<AppStateRoles<RolesRepository>>) -> Result<impl IntoResponse, impl IntoResponse> {
        let role = Roles::new("a new role".into());
        let response: Responder =   Responder::BadRequest("a bad req".to_string());
            Ok::<Responder, Responder>(response)
    }

    async fn handle_create_model(state: StateController<AppStateRoles<RolesRepository>>, body: axum::extract::Json<Roles>) -> Result<impl IntoResponse, impl IntoResponse> {

        let response = state.repository.insert(Roles::new(body.rolename.clone())).await;

        match response {
            Ok(res) => Ok(Responder::Ok(json!(res))),
            Err(db_error) => Err(Responder::DatabaseError(db_error))
        }
    }
}

