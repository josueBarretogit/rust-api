use core::panic;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{postgres::PgPoolOptions,  Pool, Postgres, Row};
use serde::{Serialize, Deserialize};
use dotenv::dotenv;




#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
struct Books {
    id : i64,
    title : String,
    review_id : Vec<i64>
}


#[derive(Debug, Serialize, Deserialize)]
struct BooksDTO {
    id : i64,
    title : String,
    reviews : Vec<Reviews>
}


#[derive(Debug, Serialize, Deserialize)]
struct Reviews {
    id : i64,
    title : String,
}

struct  AppState<T : sqlx::Database> {
    db : Pool<T>
}


#[get("/data")]
async fn get_books(app_data : web::Data<AppState<Postgres>>) -> impl Responder {
    let db = &app_data.db;

    let response  = sqlx::query("SELECT books.id as id,books.title title, ARRAY_AGG(reviews.id) as review_id from  books INNER JOIN reviews ON reviews.book_id = books.id GROUP BY books.id").fetch_all(db).await;

    match  response {
        Ok(data) => {

            let books : Vec<BooksDTO> = data.iter().map(|row| {
                let row_book = Books {
                    id : row.get("id"),
                    title : row.get("title"),
                    review_id : row.get("review_id")
                };
                let mut book :BooksDTO = BooksDTO { id: 0, title: "".to_string(), reviews: vec![
                    Reviews {
                    id : 0,
                    title : "".to_string()
                    }
                ] };

                book.id = row_book.id;
                book.title = row_book.title;

                for id in row_book.review_id {
                    book.reviews.push(Reviews {
                        id,
                        title : String::from("aaa")
                    });
                }

                book
                
            }).collect();


            HttpResponse::Ok()
                .json(web::Json(books))
        },
        Err(e) => return HttpResponse::InternalServerError().json(web::Json(e.to_string()))
    }
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error>  {

    dotenv().ok();

    let db_uri = std::env::var("DB_URI").unwrap_or_else(|err| panic!("must have a db uri {err}"));


    let pg_pool = PgPoolOptions::new().connect(&db_uri).await.unwrap_or_else(|err| panic!("couldnot connect: {err}"));



    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {db : pg_pool.clone()}))
            .wrap(middleware::Compress::default())
            .service(get_books)

    })
        .bind(("127.0.0.1" , 8080))?
        .run()
    .await


}

