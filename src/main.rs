
#[macro_use]
extern crate nickel;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

// for Nickel
use nickel::{Nickel, HttpRouter, JsonBody, MediaType, Router};
use nickel::status::StatusCode::{self};

// for json parsin
use serde::{Deserialize, Serialize};

//  for mongo
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::error::Result as MongoResult;

// for bson 
use bson::{Bson, Document};
use bson::oid::ObjectId;



#[derive(Serialize, Deserialize)]
pub struct User{
    firstname: String,
    lastname: String,
    email: String,
    password: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Atlass{
    country: String,
    city: String,
    continent: String
}


fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();
    let mut count: Router = Nickel::router();

    // for database connection
    let  conn_client = Client::connect("localhost", 27017)
        .ok().expect("Error establishing connection.");

    // selecting the database to use
    let data_get = conn_client.db("rust-users");
    let data_post = conn_client.db("rust-users");
    // let data_b = conn_client.db("rust-users");
    // let data_b = conn_client.db("rust-users");
    
    fn get_data_string(result: MongoResult<Document>) -> Result<Bson, String> {
        match result {
            Ok(doc) => Ok(Bson::Document(doc)),
            Err(e) => Err(format!("{}", e))
        }
    }

    router.get("/users", middleware! {|_request, mut _res|
        println!("Gettingt he data");
        // Connect to the database


        // The users collection
        let coll = data_get.collection("users");

        // Create cursor that finds all documents
        let cursor = coll.find(None, None).unwrap();

        // Opening for the JSON string to be returned
        let mut data_result = "{\"data\":[".to_owned();

        for (i, result) in cursor.enumerate() {
            match get_data_string(result) {
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


    router.post("/users/new", middleware! {|_req, _res|
        println!("Posting the data");
        // imported serde and serde_json to fix serde error on json_as
        let user = _req.json_as::<User>().unwrap();
        let firstname = user.firstname.to_string();
        let lastname = user.lastname.to_string();
        let email = user.email.to_string();
        let password = user.password.to_string();

        // the user connection
        let coll = data_post.collection("users");
        
        //insert one user
        match coll.insert_one(doc! {
            "firstname" => firstname,
            "lastname" => lastname,
            "email" => email,
            "password" => password
        },None){
            Ok(_) => (StatusCode::Ok, "Item Save to the database"),
            Err(e) => return _res.send(format!("Something happened => {}", e))
        }

    });

    router.put("/users/update/:id", middleware! {|_req, _res|
         println!("Updating the data");
        // imported serde and serde_json to fix serde error on json_as
        let user = _req.json_as::<User>().unwrap();
        let firstname = user.firstname.to_string();
        let lastname = user.lastname.to_string();
        let email = user.email.to_string();
        let password = user.password.to_string();
        // connect to the database
        let client = Client::connect("localhost", 27017)
            .ok().expect("Error while trying to connect");

        // the user connection
        let coll = client.db("rust-users").collection("users");
                
        let obj_id = _req.param("id").unwrap();

        let id = match ObjectId::with_string(obj_id) {
            Ok(oid) => oid,
            Err(e) => return _res.send(format!("{}", e))
        };
        
        //Delete One
        match coll.update_one(doc! { "_id" => id,}, doc!{"$set":{
            "firstname" => firstname,
            "lastname" => lastname,
            "email" => email,
            "password" => password
        }},None){
            Ok(_) => (StatusCode::Ok, "Item Updated"),
            Err(e) => return _res.send(format!("Something happened => {}", e))
        }
    });

    router.delete("/users/:id", middleware! {|_req, _res|
        let client = Client::connect("localhost", 27017).ok().expect("Failed to connect");

        let coll = client.db("rust-users").collection("users");

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

    count.get("/countries", middleware! {|_request, mut _res|
        println!("Gettingt he data");
        // Connect to the database
        let client = Client::connect("localhost", 27017)
        .ok().expect("Error establishing connection.");

        // The countries collection
        let coll = client.db("rust-users").collection("countries");

        // Create cursor that finds all documents
        let cursor = coll.find(None, None).unwrap();

        // Opening for the JSON string to be returned
        let mut data_result = "{\"data\":[".to_owned();

        for (i, result) in cursor.enumerate() {
            match get_data_string(result) {
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
        let _atlass = _req.json_as::<Atlass>().unwrap();
        let country = _atlass.country.to_string();
        let city = _atlass.city.to_string();
        let continent = _atlass.continent.to_string();

        // connect to the database
        let client = Client::connect("localhost", 27017).ok().expect("Error while trying to connect");

        // the user connection
        let coll = client.db("rust-users").collection("countries");
        
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
        let _atlass = _req.json_as::<Atlass>().unwrap();
        let country = _atlass.country.to_string();
        let city = _atlass.city.to_string();
        let continent = _atlass.continent.to_string();
        // connect to the database
        let client = Client::connect("localhost", 27017)
            .ok().expect("Error while trying to connect");

        // the user connection
        let coll = client.db("rust-users").collection("countries");
                
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

    server.utilize(router);
    server.utilize(count);
    server.listen("127.0.0.1:9000");
}