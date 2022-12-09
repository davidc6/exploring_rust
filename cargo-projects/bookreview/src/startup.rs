use std::sync::Mutex;
use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{get, post, web, App, HttpServer, Responder, Result, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::{PgPool};
use uuid::Uuid;

use crate::routes::{follows};
use crate::routes::{books};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    id: String,
    title: String,
    author: String
}

#[derive(Debug, Serialize, Deserialize)]
struct AppStateMutable {
    data: Mutex<Vec<Book>>
}

#[derive(Serialize)]
struct Health {
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Intro {
    title: String,
    author: String
}

#[get("/books")]
async fn get_books(data: web::Data<AppStateMutable>) -> Result<impl Responder> {    
    Ok(HttpResponse::Ok().json(data))
}

// #[post("/books")]
// async fn post_books(data: web::Data<AppStateMutable>, body: web::Json<Intro>) -> Result<impl Responder> {
//     let mut books = data.data.lock().unwrap();
//     books.push(Book {
//         id: "3".to_owned(),
//         title: "Title 3".to_owned(),
//         author: "Author 3".to_owned()
//     });
//     books.push(Book {
//         id: "4".to_owned(),
//         title: "Title 4".to_owned(),
//         author: "Author 4".to_owned()
//     });

//     let id = Uuid::new_v4();
//     books.push(Book {
//         id: id.to_string(),
//         title: body.title.to_owned(),
//         author: body.author.to_owned()
//     });

//     Ok(HttpResponse::Ok())
// }

#[get("/ping")]
async fn ping() -> Result<impl Responder> {
    let health = Health {
        status: "OK".to_string()
    };
    Ok(web::Json(health))
}

pub fn run(listnr: TcpListener, conn_pool: PgPool) -> Result<Server, std::io::Error> {
    let conn = web::Data::new(conn_pool);

    // temporary fake data store
    let data = web::Data::new(AppStateMutable {
        data: Mutex::new(vec![
            Book {
                id: "1".to_owned(),
                title: "Title 1".to_owned(),
                author: "Author 1".to_owned()
            },
            Book {
                id: "2".to_owned(),
                title: "Title 2".to_owned(),
                author: "Author 2".to_owned()
            }
        ]),
    });

    let addr = listnr.local_addr().unwrap();

    // Factory
    // handles transport level concerns
    // TLS, TCP socket / Unix domain, etc.
    let server = HttpServer::new(move || {
        // App is where the logic lives, routing, middlewares etc.
        App::new()
            .app_data(data.clone())
            .service(ping)
            .service(follows)
            .service(books)
            // .service(get_books)
            // .service(post_books)
            .app_data(conn.clone())
    })
    .listen(listnr)?
    .run();

    println!("Server is running on: http://{}", addr);

    Ok(server)
}
