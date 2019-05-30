// use mongodb::error::Result as MongoResult;
// use bson::{Bson, Document};
// // use bson::oid::ObjectId;
// // use rustc_serialize::json::Json;
// pub fn get_data_string(result: MongoResult<Document>) -> Result<Bson, String> {
//     match result {
//         Ok(d) => Ok(Bson::Document(doc)),
//         Err(e) => Err(format!("{}", e))
//     }
// }