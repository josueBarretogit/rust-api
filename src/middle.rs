use axum::{extract::Request, http::{ StatusCode}, middleware::Next, response::IntoResponse, RequestExt};

use crate::models::book_models::Books;


pub async fn my_middle(mut req : Request, next: Next) -> Result<impl IntoResponse, (StatusCode, String)> {

    println!("running middleware");

    let (parts, body_parts) = req.into_parts();

    let auth = match parts.headers.get("Authorization") {
        Some(a) => a,
        None => return Err((StatusCode::UNAUTHORIZED, "NO AUTHORIZADOOO".to_string())),
        
    };

    println!("the auth was: {}", auth.to_str().unwrap());

    let  new_req = Request::from_parts(parts, body_parts);

    let var_to_pass = "var to pass".to_string();

    let mut res = next.run(new_req).await;

    res.extensions_mut().insert(var_to_pass);

    Ok(res)
}
