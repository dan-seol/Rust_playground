#![allow(dead_code)]
#![allow(unused_variables)]
use http::Method;
use http::request::Request;
use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod http;
mod server;
mod website_handler;

fn main() {
    //String literal: str, String ?
    //let string = String::from("127.0.0.1:8081");
    //let string_slice = &string[10..];
    //let string_borrow: &str = &string; //automatically converted to the string slice
    //let string_literal = "1234"; //known at compile time
    //the compiler would bake it the binary itself
    // and the variable returns the string literal
    // normal String -> mutable
/* 
    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);
*/
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);
    let server = Server::new("127.0.0.1:8081".to_string());
    server.run(WebsiteHandler::new(public_path));
}


/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
 */

 // 