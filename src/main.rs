
use actix_web::{
    error, get,HttpResponse, middleware::Logger, post, web::{self, Json, ServiceConfig}, Responder, Result
};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{FromRow, PgPool, Row};
use actix_multipart::Multipart;
use futures_util::{StreamExt, TryFutureExt};

#[derive(FromRow)]
struct ImageData {
    data: Vec<u8>,
}

#[get("/{id}")]
async fn retrieve(path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {

   
    //fetching image data from postgres which was saved as bytea

    let image_data= sqlx::query_as::<_,ImageData    >("SELECT data FROM images WHERE id=$1")
        .bind(*path)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()));
    
    println!("Tried fetching data");
   match image_data{
    Ok(data)=> HttpResponse::Ok()
    .content_type("image/jpeg")
    .append_header((
        "Content-Disposition",
        format!("inline; filename=\"{}\"", "hola"),
    ))
    .body(data.data),
    Err(e)=> HttpResponse::BadRequest().body(e.to_string()),
   }
       
    


   
}

#[post("")]
async fn add( mut payload:Multipart, state: web::Data<AppState>) -> Result<impl Responder, actix_web::Error> {

    while let Some(field) = payload.next().await {  
        let mut field = field.unwrap();

        let mut image_data = Vec::new();
        while let Some(chunk) = field.next().await {
            let chunk = chunk.unwrap();
            image_data.extend_from_slice(&chunk);
        }

        sqlx::query("INSERT INTO images (data) VALUES ($1)")
       .bind(image_data)
       .execute(&state.pool)
       .await
       .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
        
       
    }
    println!("Data added into db");
    Ok(HttpResponse::Ok().body("Data added successfully"))
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = web::Data::new(AppState { pool });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .wrap(Logger::default())
                .service(retrieve)
                .service(add)
                .app_data(state),
        );
    };

    Ok(config.into())
}


