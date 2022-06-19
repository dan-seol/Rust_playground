# Building an HTTP server
- Learning Rust is prioritized over making the server perform well

## HTTP/1.1
- L7 protocol
- Sent over TCP
- Message based
- Request sent, Response receive

### REQUEST
- e.g. GET /search?name=abc HTTP/1.1
Accept: text/html

### RESPONSE
HTTP/1.1 200 OK
Content-Type:text/html

<!DOCTYPE html>
<html lang="en">
...
</html>

Architecture

Server{TCPListener, HTTPParser, Handler}

## Strings in Rust
Rust has two types of Strings
str : immutable reference to a string slice
string view inside of an existing string

String and &str

Suppose we have allocated some strings on the heap

String s

on stack

String
(name, value) : [length:10, capacity:12, ptr: s]

if we want to take the part after a whitespace
one way: copy that part to a new heap memory

&str, just return the pointer of letter after the space

Rust strings can only store utf-8s

anti pattern do not choose hard-coded index to string slice

Rust enums can each hold values of different type

## Option

Rust does not have null value
as trying to use null like not null can cause great problems

So it extensively uses Optional type implementation `Option<T>` as a enum!