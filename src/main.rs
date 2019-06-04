#[macro_use]
extern crate nickel;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;

// for bson and mongo drivers
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

// for authenticatiion
extern crate jwt;
extern crate hyper;
extern crate crypto;

// for Nickel
use nickel::{Nickel, HttpRouter, JsonBody, MediaType, MiddlewareResult, Request, Response,};
use nickel::status::StatusCode::{self, Forbidden};


//  for mongo
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

// for bson
use bson::oid::ObjectId;

// for auth
use hyper::header;
use hyper::header::{Authorization, Bearer};


// jwt
use std::default::Default;
use crypto::sha2::Sha256;
use jwt::{Header, Registered, Token};

//for the structs
pub mod models;
pub mod countries;

static AUTH_SECRET: &'static str = "this  be my key";


fn auth<'mw>(request: &mut Request, response: Response<'mw>, ) -> MiddlewareResult<'mw>{
    if request.origin.method.to_string() == "OPTION".to_string(){
        return response.next_middleware();
    } else {
        // We do not want to apply the middleware to the login route
        if request.origin.uri.to_string() == "/login".to_string() {
            return response.next_middleware();
        }else{
                // Get the full Authorization header from the incoming request headers
            let auth_header = match request.origin.headers.get::<Authorization<Bearer>>() {
                Some(header) => header,
                None => panic!("No authorization header found")
            };

            // Format the header to only take the value
            let jwt = header::HeaderFormatter(auth_header).to_string();

            // We don't need the Bearer part,
            // so get whatever is after an index of 7
            let jwt_slice = &jwt[7..];

            // Parse the token
            let token = Token::<Header, Registered>::parse(jwt_slice).unwrap();

            // Get the secret key as bytes
            let secret = AUTH_SECRET.as_bytes();

            // Verify the token
            if token.verify(&secret, Sha256::new()) {
                return response.next_middleware();
            } else {
                return response.error(Forbidden, "Access denied");
            }
        }
    }
}


fn main() {
    // countries::countries();
    let mut server = Nickel::new();
    let mut router = Nickel::router();
    // let mut count: Router = Nickel::router();

    // for database connection
    let  conn_client = Client::connect("localhost", 27017)
        .ok().expect("Error establishing connection.");

    // selecting the database to use
    let data_get = conn_client.db("rust-users");
    let data_post = conn_client.db("rust-users");
    let data_for_login = conn_client.db("rust-users");
    // let data_b = conn_client.db("rust-users");
    

    router.post("/login", middleware! { |request|

        // The users collection
        let colle = data_for_login.collection("users");
        // Accept a JSON string that corresponds to the User struct
        let log_user = request.json_as::<models::LoginStuff>().unwrap();

        // Get the email and password
        let email = log_user.email.to_string();
        let password = log_user.password.to_string();

        let cur =  colle.find(Some(doc! { "password"=> &password, "email" => &email }), None).unwrap();
        let pass = cur.count();
          
        // Simple password checker
        if pass == 1{

            let header: Header = Default::default();
            // For the example, we just have one claim
            // You would also want iss, exp, iat etc
            let claims = Registered {
                sub: Some(email.into()),
                ..Default::default()
            };

            let token = Token::new(header, claims);

            // Sign the token
            let jwt = token.signed(AUTH_SECRET.as_bytes(), Sha256::new()).unwrap();

            format!("this is my secrete key => {}", jwt)

        } else {
            format!("Incorrect username or password")
        }

    });

    router.get("/users", middleware! {|_request, mut _res|
        println!("Gettingt the data");
        
        let coll = data_get.collection("users"); // The users collection
        
        let cursor = coll.find(None, None).unwrap(); // Create cursor that finds all documents
       
        let mut data_result = "{\"data\":[".to_owned(); // Opening for the JSON string to be returned

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

        data_result.push_str("]}"); // Close the JSON string
        
        _res.set(MediaType::Json); // Set the returned type as JSON

        format!("{}", data_result) // Send back the result
    });

    router.post("/users/new", middleware! {|_req, _res|
        println!("Posting the data");
        // imported serde and serde_json to fix serde error on json_as
        let user = _req.json_as::<models::User>().unwrap();
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
        let user = _req.json_as::<models::User>().unwrap();
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
        
        //Update One
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

   

    server.utilize(auth);
    server.utilize(router);
    server.utilize(countries::countries());
    server.listen("127.0.0.1:9000");
}