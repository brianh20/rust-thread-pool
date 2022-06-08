# Multi-Threaded Web Server

This web server implements a thread pool of Worker struct(s) which receives a Message containing a Job or an instruction to Terminate.

## Running
- `cargo run` to run
- Call the API at 127.0.0.1:7878

## Crate Structure
- connection.rs: starts the listener using the thread pool
- thread_pool.rs: implements the thread pool of workers, instatiate and termination
- worker.rs: implements each worker
- message.rs: has the message types which the worker receives

# As a Rust excercise

This repository aims to display the implementation of a number of Rust features/paradigms. This means that perhaps some things could have been different, but there were some "artistic liberties" taken in order to make use of these features/paradigms. A number of them are:
- There is no action taken other than a println! of the received request
- The listeners are limited to 2 calls and then shutdown. This is to illustrate the Termination message. In order to remove this search and remove `.take(2)` in `connection.rs`

## From the rust book
Topics taken from "Programming Rust 2nd Edition".
https://www.amazon.es/Programming-Rust-Fast-Systems-Development/dp/1492052590

This builds on knowledge/learning/practice from the Elevator Excercise and adds:
- Thread pool
- Workers
- Smart Pointers (RC, ARC)

Pending for a follow-up excercise:
- gRPC
- Asynchronous programming
- Macros

