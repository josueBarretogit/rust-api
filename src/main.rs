use core::panic;
use actix_web::{body::BoxBody, get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use serde::{Serialize, Deserialize};
use dotenv::dotenv;



#[derive(Debug, Serialize, Deserialize)]
struct Books {
    title : String
}


impl Responder for Books {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}


struct  AppState {
    db : Pool<Postgres>
}


#[get("/data")]
async fn get_books(app_data : web::Data<AppState>) -> impl Responder {
    let db = &app_data.db;

    let res  = sqlx::query("SELECT title from  books where id = 1").fetch_one(db).await.unwrap();

    let book = Books {
        title : res.get("title")
    };

    book
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error>  {

    dotenv().ok();
    let db_uri = std::env::var("DB_URI").unwrap_or_else(|err| panic!("must have a db uri {err}"));


    let pg_pool = PgPoolOptions::new().connect(&db_uri).await.unwrap_or_else(|err| panic!("couldnot connect: {err}"));


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {db : pg_pool.clone()}))
            .service(get_books)

    })
        .bind(("127.0.0.1" , 8080))?
        .run()
    .await


}

