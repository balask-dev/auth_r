use actix_web::{web::Data, App, HttpServer,HttpResponse};
use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users}; //import the handler here
use rep::mongod::Mongo;

 
mod api; 
mod models;
mod rep;

#[get("/")]
async fn hello()-> impl Responder{
    HttpResponse::Ok().json("hello from rust")
}




#[actix_web::main]
async fn main()-> std::io::Result<()>{
    let db = Mongo::init().await;
    let db_data = Data::new(db);

HttpServer::new(move || 
    App::new().app_data(db_data.client()).service(create_user))
    .bind(("localhost",8000))?.run().await
 
}

