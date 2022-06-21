use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
use crate::http::Request; //crate -> root of the entire project
//read is trait

pub struct Server {
    addr: String
}

fn arr(a: &[u8]) {

}

impl Server {
     //methods -> similar to functions, but under the context of struct
    pub fn new(addr: String) -> Self { //Alias for the name of the struct
        Self { addr }
    } 
    
    pub fn run(self) { //if not passed by ref, self deallocates
        println!("Listening on {}", self.addr);
        //remark that the bind(...) returns a result of TcpListener
        // Result<T> type is the key of error handling logic in rust
        let listener =  TcpListener::bind(&self.addr).unwrap();
        /*
        Errors in rust -> recoverable & irrecoverable 
        recoverable -> file not found
        irrecoverable -> trying to access an index beyond the array
        other langs do not distinguish these two errors, all wrapped in exception
        rust does not have exception
        Result<T, E> is defined as
        pub enum Result<T, E> {
            Ok(T),
            Err(E),
        }
        for bind(...) might give the error if the socket is already in use, etc.
        Result enum has the function called .unwrap()
        unwrap() -> the actual result if successful, terminate the program otherwise
        */
        // rust has a special type of keyword used for inf loop
        // in rust iterative clauses can be named
        /*
        'outer: loop {
           loop {
               break 'outer;
           }
       }
       */
        loop {
          /*
          accept a new incoming connection
          from this listener
          returns Result<(TcpStream, SocketAddr)>
           */
          /* a suboptimal way to handle the result
          let res = listener.accept();
          
          if (res.is_err()) {
              continue;
          }

          let (stream, addr) = res.unwrap();

          we can use match keyword to do pattern matching
          match in rust can perform both as switch statements and pattern matching on enum
          */
          match listener.accept() {
              Ok((mut stream, _)) => {
                //let a = [1,2,3,4,5]; 
                //arr(&a[1..3]); 
                //let a = 5;
                  //println!("OK");
                  //need explicit initial values
                  // vs C: you will get 1024 bytes allocated, with storing whatever values 
                  // there were previously
                  let mut buffer = [0; 1024];

                  match stream.read(&mut buffer) {
                      Ok(_) => {
                          //unlike String::from_utf8, this never fails
                          //{:?} <- use debug implementation
                          println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                          //&buffer as &[u8]
                          match Request::try_from(&buffer[..]) {
                              Ok(request) => {}
                              Err(e) => println!("Failed to parse a request: {}", e)
                          }
                          //let res: &Result<Request, _> = &buffer[..].try_into();
                      },
                      Err(e) => println!("Failed to read from connection: {}", e),
                  }
              },
              Err(e) => println!("Failed to establish a connection: {}", e),
              // _ => .. default case
              
          }
        }
        //let tup = (5, "a", listener);
        // match "abcd" {
              // a | b => {}
              //...
        //}

      /*
      what do we do when we do not want the program to fail 
      when we got an error from a result?
       */
    }
}
