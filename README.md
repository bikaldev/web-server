# Multi-Threaded Web Server in Rust

This project is a simple multi-threaded web server written in Rust. The server listens on port `7878` and serves web pages at specified paths. It demonstrates basic concepts of networking, threading, and HTTP handling in Rust.

## Features

- **Multi-Threaded:** The server handles requests using a thread pool, allowing it to manage multiple connections concurrently.
- **Routing:** 
  - Serves a page at `/`.
  - Simulates a delayed response at `/sleep`.
  - Returns a `404 Not Found` error for any other path.
  
## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (Ensure you have the latest stable version installed)

### Building the Project

1. Clone the repository:
    ```sh
    git clone https://github.com/bikaldev/multi-threaded-web-server.git
    cd multi-threaded-web-server
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

### Running the Server

After building the project, you can run the server using:

```sh
cargo run
