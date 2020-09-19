// http://diesel.rs/guides/getting-started/
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, post};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::Deserialize;
use self::models::*;
use diesel::r2d2::*;

use dotenv::dotenv;
use std::env;

//Sqliteコネクションを作る。
pub fn pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().max_size(15).build(manager).unwrap()
}

use self::schema::posts::dsl::*;
pub fn create_post<'a>(conn: &SqliteConnection, other_title: &'a str, other_body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: other_title,
        body: other_body,
        published: &true,
    };

    diesel::insert_into(posts::table)
        .values(&new_post).execute(conn)
        .expect("Error saving new post");

        posts.order(id.desc()).first(conn).unwrap()
}

#[derive(Deserialize)]
struct InfoQuery {
    q: String,
}

async fn index(info_query: web::Query<InfoQuery>) -> impl Responder {
    HttpResponse::Ok().body(format!("{} san! {}", &info_query.q, "Hello World!"))
}

#[get("/foo")]
async fn index2(db: web::Data<Pool<ConnectionManager<SqliteConnection>>>) -> impl Responder {
    use self::schema::posts::dsl::*;

    let connection = db.get().unwrap();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    let mut s = String::new();
    for post in results {
        s += &post.title.to_string();
    }
    HttpResponse::Ok().json(s)
}

#[derive(Deserialize)]
struct Info {
    title: String,
    body: String,
}

#[post("/bar")]
async fn post_title(info: web::Json<Info>, db: web::Data<Pool<ConnectionManager<SqliteConnection>>>) -> impl Responder {
    let conn = db.get().unwrap();
    let ret = create_post(&conn, &info.title, &info.body);
    HttpResponse::Ok().body(ret.title + ":" + &ret.body + " が登録されました\n")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {


    HttpServer::new(|| {
        let pool = pool();
        App::new()
        .data(pool)
        .route("/", web::get().to(index))
        .service(index2)
        .service(post_title)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
