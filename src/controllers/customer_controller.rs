use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

use crate::{models::customer_model::Customer, repositories::Repository, AppStateCustomer};

use super::Controller;



pub struct CustomerController {}



impl Controller<Customer, AppStateCustomer> for CustomerController {

    async fn handle_get_models(state: axum::extract::State<AppStateCustomer>) -> Result<Json<Vec<Customer>>, (StatusCode, Json<Value>)> {
        let response = state.repository.find_all().await;
        match response {
            Ok(customers) => Ok(Json(customers)),
            Err(_e) => Err((StatusCode::BAD_REQUEST, Json(json!({"response" : "bad request"}))))
        }
    }
    

    async fn handle_create_model(body : axum::extract::Json<Customer> ) -> impl axum::response::IntoResponse {
        Json(json!({"customer" : Customer::new("asdasdj".to_string())}))
    }

    

    
}
