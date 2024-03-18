mod config {
    use serde::Deserialize;
    #[derive(Debug, Default, Deserialize)]
    pub struct ExampleConfig {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "books")] // singular 'user' is a keyword..
    pub struct Books {
        pub id : i64,
        pub title: String,
        pub cover_photo_url :String,
        pub description: String,
        pub rating: i64,
    }
}

mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }
    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

mod db {
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    use crate::{errors::MyError, models::Books};

    pub async fn get_books(client: &Client) -> Result<Vec<Books>, MyError> {
        let stmt = client.prepare("SELECT id, title, cover_photo_url, description, rating from books where id = 1").await.unwrap();

        let results = client
            .query(&stmt, &[])
            .await?
            .iter()
            .map(|row| Books::from_row_ref(row).unwrap())
            .collect::<Vec<Books>>();

        Ok(results)
    }

    
}

mod handlers {
    use actix_web::{web, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};

    use crate::{db, errors::MyError};

    pub async fn get_books(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let users = db::get_books(&client).await?;

        Ok(HttpResponse::Ok().json(users))
    }

}

use ::config::Config;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use handlers::{get_books};
use tokio_postgres::NoTls;

use crate::config::ExampleConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::resource("/users")
                .route(web::get().to(get_books)),
        )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}

