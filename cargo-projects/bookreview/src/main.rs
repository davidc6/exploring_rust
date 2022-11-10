use std::sync::Mutex;
use actix_web::{get, post, web, App, HttpServer, Responder, Result, HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    id: String,
    title: String,
    author: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Intro {
    title: String,
    author: String
}

#[derive(Serialize)]
struct Health {
    status: String,
}

#[derive(Debug, Serialize)]
struct AppStateNew {
    data: Vec<Book>
}

#[derive(Serialize, Deserialize, Debug)]
struct AppStateMutable {
    data: Mutex<Vec<Book>>
}

struct AppState {
    str: String
}

#[get("/books")]
async fn get_books(data: web::Data<AppStateMutable>) -> Result<impl Responder> {    
    Ok(HttpResponse::Ok().json(data))
}

#[post("/books")]
async fn post_books(data: web::Data<AppStateMutable>, body: web::Json<Intro>) -> Result<impl Responder> {
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

    let id = Uuid::new_v4();
    books.push(Book {
        id: id.to_string(),
        title: body.title.to_owned(),
        author: body.author.to_owned()
    });

    Ok(HttpResponse::Ok())
}

#[get("/ping")]
async fn ping() -> Result<impl Responder> {
    let health = Health {
        status: "OK".to_string()
    };
    Ok(web::Json(health))
}

// async runtime is loaded on top of the main fn
// and used to drive futures (async computations) to completion
// tokio runtime takes async code in the main fn and runs it
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

    // Factory
    // handles transport level concerns
    // TLS, TCP socket / Unix domain, etc.
    HttpServer::new(move || {
        // App is where the logic lives, routing, middlewares etc.
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
