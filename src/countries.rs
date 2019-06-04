
use nickel::{Nickel, HttpRouter, JsonBody, MediaType, Router};
use nickel::status::StatusCode::{self};

//  for mongo
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

// for bson
use bson::oid::ObjectId;
// mod models;
use crate::models;

// static count: Router = Nickel::router();

pub fn countries() -> Router{
    let mut count: Router = Nickel::router();
      // for database connection
    let  conn_client = Client::connect("localhost", 27017)
        .ok().expect("Error establishing connection.");

    // selecting the database to use
    let data_get = conn_client.db("rust-users");
    let data_post = conn_client.db("rust-users");
    let data_put = conn_client.db("rust-users");

    count.get("/countries", middleware! {|_request, mut _res|
        println!("Gettingt he data");
        // Connect to the database
        

        // The countries collection
        let coll = data_get.collection("countries");

        // Create cursor that finds all documents
        let cursor = coll.find(None, None).unwrap();

        // Opening for the JSON string to be returned
        let mut data_result = "{\"data\":[".to_owned();

        for (i, result) in cursor.enumerate() {
            match models::get_data_string(result) {
                Ok(data) => {
                    let string_data = if i == 0 {
                        format!("{}", data)
                    } else {
                        format!("{},", data)
                    };

                    data_result.push_str(&string_data);
                },

                Err(e) => return _res.send(format!("{}", e))
            }
        }

        // Close the JSON string
        data_result.push_str("]}");

        // Set the returned type as JSON
        _res.set(MediaType::Json);

        // Send back the result
        format!("{}", data_result)
    
    });

    count.post("/countries/new", middleware! {|_req, _res|
        println!("Posting the data");
        // imported serde and serde_json to fix serde error on json_as
        let _atlass = _req.json_as::<models::Atlass>().unwrap();
        let country = _atlass.country.to_string();
        let city = _atlass.city.to_string();
        let continent = _atlass.continent.to_string();

        // the user connection
        let coll = data_post.collection("countries");
        
        //insert one user
        match coll.insert_one(doc! {
            "country" => country,
            "city" => city,
            "continent" => continent
        },None){
            Ok(_) => (StatusCode::Ok, "Item Save to the database"),
            Err(e) => return _res.send(format!("Something happened => {}", e))
        }

    });

    count.put("/countries/update/:id", middleware! {|_req, _res|
        println!("Updating the data");
        // imported serde and serde_json to fix serde error on json_as
        let _atlass = _req.json_as::<models::Atlass>().unwrap();
        let country = _atlass.country.to_string();
        let city = _atlass.city.to_string();
        let continent = _atlass.continent.to_string();


        // the user connection
        let coll = data_put.collection("countries");
                
        let obj_id = _req.param("id").unwrap();

        let id = match ObjectId::with_string(obj_id) {
            Ok(oid) => oid,
            Err(e) => return _res.send(format!("{}", e))
        };
        
        //Delete One
        match coll.update_one(doc! { "_id" => id,}, doc!{"$set":{
            "country" => country,
            "city" => city,
            "continent" => continent
        }},None){
            Ok(_) => (StatusCode::Ok, "Item Updated"),
            Err(e) => return _res.send(format!("Something happened => {}", e))
        }
    });

    count.delete("/countries/delete/:id", middleware! {|_req, _res|
        let client = Client::connect("localhost", 27017).ok().expect("Failed to connect");

        let coll = client.db("rust-users").collection("countries");

        let obj_id = _req.param("id").unwrap();

        let id = match ObjectId::with_string(obj_id) {
            Ok(oid) => oid,
            Err(e) => return _res.send(format!("{}", e))
        };

        match coll.delete_one(doc! {"_id" => id}, None){
            Ok(_) => (StatusCode::Ok, "Item deleted"),
            Err(e) => return _res.send(format!("Something happened while trying to delete => {}", e))

        }
    });
    count
}