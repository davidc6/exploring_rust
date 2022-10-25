use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpServer, Responder, Result, HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};

// use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    id: String,
    title: String,
    author: String
}

#[get("/books")]
async fn get_books(data: web::Data<AppStateMutable>) -> Result<impl Responder> {    
    let mut books = data.data.lock().unwrap();
    books.push(Book {
        id: "3".to_owned(),
        title: "Title 3".to_owned(),
        author: "Author 3".to_owned()
    });
    books.push(Book {
        id: "4".to_owned(),
        title: "Title 4".to_owned(),
        author: "Author 4".to_owned()
    });
    Ok(HttpResponse::Ok().json(&**books))
}

#[post("/books")]
async fn post_books(req: HttpRequest, counter: web::Data<AppState>) -> impl Responder {
    // TODO: unique id for a book
    // let a = *req.app_data::<AppStateWithCounter>().unwrap();
    // println!("{:?}", a);
    // counter.counter.set(counter.counter.get());

    "POST Books endpoint"
}

#[derive(Serialize)]
struct Health {
    status: String,
}

#[get("/ping")]
async fn ping() -> Result<impl Responder> {
    let health = Health {
        status: "OK".to_string()
    };
    Ok(web::Json(health))
}

// #[derive(Debug, Clone, Copy)]
struct AppState {
    str: String
}

#[derive(Debug, Serialize)]
struct AppStateNew {
    data: Vec<Book>
}

#[derive(Serialize, Deserialize, Debug)]
struct AppStateMutable {
    data: Mutex<Vec<Book>>
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
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

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(ping)
            .service(get_books)
            .service(post_books)
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}
