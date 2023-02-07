use mongodb::{bson,results,Client,Collection};
extern crate dotenv;
use std::env;
use dotenv::dotenv;
use models::user_models::User;

pub struct Mongo{
    collec:Collection<User>,
}

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc}, 
    results::{ InsertOneResult, UpdateResult, DeleteResult},
    Client, Collection,
}

impl Mongo{
    pub async fn init()->self{
        dotenv.ok();
        let url = match env::var("URL"){
            Ok(e)=> e.to_string(),
            Err(_)=> format!("Configuration Error"),
        };
        let client = Client::with_url_str(url).unwrap();
        let db = client.databse("RuApp");
        mongo{collec}

    }


pub async fn create_user(new_user:User)->Result<InsertOneResult,Error>{
  let new_doc = User{
    id:None,
    name:new_user.name,
    college:new_user.college,
    dept:new_user.dept
  };

  let user = self.collec.insert_one(new_doc,None).ok().await.
             .expect("Error while creating user");
             Ok(user)

}

pub async fn get_user(&self, id: &String) -> Result<User, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let user_detail = self
        .collec
        .find_one(filter, None)
        .await
        .ok()
        .expect("Error getting user's detail");
    Ok(user_detail.unwrap())
}


pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let new_doc = doc! {
        "$set":
            {
                "id": new_user.id,
                "name": new_user.name,
                "college": new_user.college,
                "dept": new_user.dept
            },
    };
    let updated_doc = self
        .collec
        .update_one(filter, new_doc, None)
        .await
        .ok()
        .expect("Error updating user");
    Ok(updated_doc)
}

pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let user_detail = self
        .collec
        .delete_one(filter, None)
        .await
        .ok()
        .expect("Error deleting user");
    Ok(user_detail)
}
}

