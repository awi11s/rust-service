extern crate diesel;
use actix_web::{
    get,
    Responder,
    HttpResponse,
    Error,
    web
};
use diesel::{ 
    pg::PgConnection, 
    r2d2::{ ConnectionManager, Pool },
    prelude::*,
};


pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index(
    pool: web::Data<DbPool>, 
    name: web::Path<(String)>
) -> impl Responder {

    let name = name.into_inner();
    let conn = pool.get().expect("could not connect to db");
    
    let user = web::block(move || actions::insert_new_user(&conn, &user))
        .await.map_err(|e| { 
            println!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json("Rust service prototype"))
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

#[get("/song/{song_id}")]
pub async fn get_song(
    pool: web::Data<DbPool>, 
    song_id: web::Path<i32>
) -> impl Responder {
    use schema::songs::dsl::*;

    let song_id = song_id.into_inner();
    let conn = pool.get().expect("could not connect to db");
    
    let vulnerability = web::block(move || songs::insert_new_song(&conn, &song_id))
        .await.map_err(|e| { 
            println!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(vulnerability) = vulnerability {
        Ok(HttpResponse::Ok().json(vulnerability))
    } else {
        let res = HttpResponse::NotFound().body(format!("No vuln found with id: {}", vuln_id));
        Ok(res)
    }

}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(index)
            .service(healthcheck)
    );
}

