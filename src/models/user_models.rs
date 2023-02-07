use serde::{Serialize,Deserialize};
//use mongodb::bson::oid::ObjectId;

#[derive(Serialize,Deserialize,Debug)]
pub struct User{
    pub id:<Option<ObjectId>,
    pub name:String,
    pub college:String,
    pub dept:String, 
}