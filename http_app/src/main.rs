use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;

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


    let server = Server::new("127.0.0.1:8081".to_string());
    server.run();
}


/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
 */

 // 