#[macro_use]
extern crate nickel;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;


// use serde::{Deserialize, Serialize};

// for Nickel
use nickel::{Nickel, HttpRouter, JsonBody};
// use nickel::{Nickel, JsonBody, HttpRouter, MediaType};
use nickel::status::StatusCode::{self};

// for json parsin
use serde::{Deserialize, Serialize};
// use serde_json::Deserializer;



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


fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    // router.get("/users", middleware! {|request, response|
    //     format!("Hello from GET/users")
    // });


    router.post("/users/new", middleware! {|request, response|
        
        // imported serde and serde_json to fix serde error on json_as
        let user = request.json_as::<User>().unwrap();
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
            Err(e) => return response.send(format!("Something happened => {}", e))
        }

    });


    router.delete("/users/:id", middleware! {|request, response|
        format!("Hello from DELETE/users/:id")
    });

    server.utilize(router);
    server.listen("127.0.0.1:9000");
}
