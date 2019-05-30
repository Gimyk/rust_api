// mod code;
#[macro_use]
extern crate nickel;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

// for Nickel
use nickel::{Nickel, HttpRouter, JsonBody, MediaType};
use nickel::status::StatusCode::{self};

// for json parsin
use serde::{Deserialize, Serialize};
// use serde_json::Deserializer;
// use serde_json::json;
//  for mongo
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::error::Result as MongoResult;

// for bson 
use bson::{Bson, Document};
use bson::oid::ObjectId;

// for rustc_serialize
use rustc_serialize::json::{Json, ToJson};

// #[derive(Deserialize)]
#[derive(Serialize, Deserialize, Debug)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct User{
    firstname: String,
    lastname: String,
    email: String,
    password: String
}

fn get_data_string(result: MongoResult<Document>) -> Result<Bson, String> {
    match result {
        Ok(doc) => Ok(Bson::Document(doc)),
        Err(e) => Err(format!("{}", e))
    }
}

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/users", middleware! {|_request, mut _res|
        // Connect to the database
        let client = Client::connect("localhost", 27017)
        .ok().expect("Error establishing connection.");

        // The users collection
        let coll = client.db("rust-users").collection("users");

        // Create cursor that finds all documents
        let mut cursor = coll.find(None, None).unwrap();

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
        
        // imported serde and serde_json to fix serde error on json_as
        let user = _req.json_as::<User>().unwrap();
        let firstname = user.firstname.to_string();
        let lastname = user.lastname.to_string();
        let email = user.email.to_string();
        let password = user.password.to_string();
        // connect to the database
        let client = Client::connect("localhost", 27017).ok().expect("Error while trying to connect");

        // the user connection
        let coll = client.db("rust-users").collection("users");
        
        //insert one user
        match coll.insert_one(doc! {
            "firstname" => firstname,
            "lastname" => lastname,
            "email" => email,
            "password" => password
        },None){
            Ok(_) => (StatusCode::Ok, "Item Save to the databas"),
            Err(e) => return _res.send(format!("Something happened => {}", e))
        }

    });


    router.delete("/users/:id", middleware! {|request, response|
        format!("Hello from DELETE/users/:id")
    });

    server.utilize(router);
    server.listen("127.0.0.1:9000");
}


