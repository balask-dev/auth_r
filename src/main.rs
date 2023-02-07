use actix_web::{get, web,App,HttpServer,HttpResponse,Responder};
pub mod user_model;
mod api; 
mod models;
mod repository;

#[get("/")]
async fn hello()-> impl  Responder{
    HttpResponse::Ok().println!("hello from rust")
}




#[actix_web::main]
async fn main()-> std::io::Result<()>{
HttpServer::new(|| 
    App::new().service(hello)).bind(("localhost",8000))?.run().await
 
}

