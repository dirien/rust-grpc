# Creating a microservice in Rust using gRPC

## TL;DR Le Code

%[https://github.com/dirien/rust-grpc] 

## Introduction

In today's `Rust` 🦀 tutorial, we will be discovering the world of gRPC. For this, we will create a very simple microservice with a single endpoint which will echo back the message we send to it. To test our microservice, we're also going to create a simple `Rust` 🦀 client. We are also going to use some of the features we learned in the previous blog posts from my series on `Rust` 🦀.

If you haven't read the previous blog posts, I recommend you to go ahead and read them before:

%[https://blog.ediri.io/lets-build-a-cli-in-rust] 

%[https://blog.ediri.io/how-to-asyncawait-in-rust-an-introduction] 

## Prerequisites

Before we start, we need to make sure we have the following tools installed:

*   [Rust](https://www.rust-lang.org)

*   An IDE or text editor of your choice

*   Protocol Buffer Compiler (protoc)


### Install protoc

To generate the gRPC code, we need to install the `protoc` compiler. You can find the installation instructions for your platform [here](https://grpc.io/docs/protoc-installation/).

If you are on macOS, you can install it using Homebrew:

```bash
brew install protobuf
```

Ensure that the `protoc` compiler is available in your `PATH`:

```bash
protoc --version # should print the version
# libprotoc 3.21.9
```

Now we have all set up, let's get started and talk a bit about: *What is gRPC 📡?*

## What is gRPC 📡?

In gRPC, a client application can directly call methods on a server application on a different machine as if it was a local object, making it easier to create distributed applications and services. On the server side, the server implements the service and runs a gRPC server to handle client calls. On the client side, the client has a stub that provides the same methods as the server.

### Protocol Buffers

Protocol Buffers are Google's language-neutral, platform-neutral, extensible mechanism for serializing structured data and are used by gRPC by default.

Let me give you an example of how **Protocol Buffers** work. The first step is to define the data structure in a file with a `.proto` extension. Protocol buffer data is structured in messages, which are collections of named fields. Here is a very simplified example of a message:

```plaintext
message Weather {
  string city = 1;
  int32 temperature = 2;
}
```

Once we have defined our message, we can use the `protoc` compiler to generate the data access classes in your preferred language from your proto definition. The generated classes will have accessors for each of the fields in the message.

You can define gRPC services in the same `.proto` file as the messages, with RPC methods that use those messages.

```plaintext
service WeatherService {
  rpc GetWeather (WeatherRequest) returns (WeatherResponse) {}
}

message WeatherRequest {
  string city = 1;
}

message WeatherResponse {
  string forecast = 1;
}
```

You can then use the `protoc` compiler to generate the gRPC client and server interfaces from your `.proto` service.

## **Building the Microservice in Rust 🦀**

### Create a New Project

Let's start by creating a new project using the `cargo` command:

```bash
cargo new
```

### Add CLI Support

We are going to use the `clap` crate to add CLI support to our microservice and client. Add the dependency to your project via following command:

```bash
cargo add clap --features derive
```

### Create the Proto File

Now we create a new directory called `proto` and inside this folder we create a new file called `echo.proto` in it. Here we define our service and the messages we want to use:

```plaintext
syntax = "proto3";
package api;

message EchoRequest {
  string message = 1;
}

message EchoResponse {
  string message = 1;
}

service EchoService {
  rpc Echo(EchoRequest) returns (EchoResponse);
}
```

### Generate the Rust 🦀 Code From the Proto File

To generate the `Rust` 🦀 code from the proto file, we're going to use the `tonic-build` crate. We need to add it to our project as a build dependency via following command:

```bash
cargo add tonic-build --build
```

Now we can add the following code to our `build.rs` file:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/echo.proto")?;
    Ok(())
}
```

Normally, you would run `cargo build` to generate the `Rust` 🦀 code from the proto file. In IntelliJ IDEA, you may have to activate `org.rust.cargo.evaluate.build.scripts` in the `Experimental Features` settings to make it work.

`tonic-build` is part of the `tonic` crate, which is a gRPC over HTTP/2 implementation focused on high performance, interoperability, and flexibility. It's built on top of `hyper`, `tokio`, and `prost`.

Some features of `tonic` are:

*   Bi-directional streaming

*   High performance async io

*   Interoperability

*   TLS backed by rustls

*   Load balancing

*   Custom metadata

*   Authentication

*   Health Checking


Finally, we're going to add `tokio` as a dependency to our project:

```bash
cargo add tokio --features macros,rt-multi-thread
```

Now with all the gRPC 📡 code generated, we can start implementing our microservice.

### Implement the Microservice

Let's start by creating a new file called `server.rs.rs` in our `src` directory. Here we're going to implement our server logic.

First, we need to import the generated code from our proto file, as well as the `tonic` and `clap` crates:

```rust
use tonic::{transport::Server, Request, Response, Status};

use api::echo_service_server::{EchoService, EchoServiceServer};
use api::{EchoRequest, EchoResponse};

use ::clap::{Parser};
```

We also need to include the generated proto server and client items using the `include_proto!` macro:

```rust
pub mod api {
    tonic::include_proto!("api");
}
```

Now we can implement the service logic of our microservice. We're going to implement the `Echo` method of our service, which will echo back the message we send to it. We use here the `async` keyword to make the function asynchronous and `#[tonic::async_trait]` to make it compatible with `tonic`. If you want to learn more about `async`/`await` in `Rust` 🦀, feel free to check out my previous blog post: ---

```rust
#[derive(Debug, Default)]
pub struct Echo {}

#[tonic::async_trait]
impl EchoService for Echo {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = EchoResponse {
            message: format!("{}", request.into_inner().message),
        };

        Ok(Response::new(reply))
    }
}
```

Now we can start our server and listen for incoming requests, we're going to use `clap` to configure our server host and port. Details on how to use `clap` can be found in my previous article ---

```rust
#[derive(Parser)]
#[command(author, version)]
#[command(about = "echo-server - a simple echo microservice", long_about = None)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ServerCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse()?;
    let echo = Echo::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(EchoServiceServer::new(echo))
        .serve(addr)
        .await?;

    Ok(())
}
```

Next, we define a `bin` target in our `Cargo.toml` file:

```toml
[[bin]]
name = "echo-server"
path = "src/server.rs"
```

Start our server:

```bash
cargo run --bin echo-server
```

And you should see the following output:

```bash
Server listening on 127.0.0.1:50052
```

You can configure the server host and port using the `--server` and `--port` flags:

```bash
cargo run --bin echo-server -- --server 0.0.0.0 --port 50051
```

### Implement the Client

For the client, we will add the following lines to our existing `main.rs` file. First, we need to import the generated code from our proto file, as well as the `clap` crate for parsing command line arguments:

```rust
use api::echo_service_client::EchoServiceClient;
use api::EchoRequest;
use ::clap::{Parser};

pub mod api {
    tonic::include_proto!("api");
}
```

Similar to the server, we're going to use `clap` to configure our client host and port. But we are going to use the argument called `message` to send a message of our choice to the server:

```rust
#[derive(Parser)]
#[command(author, version)]
#[command(about = "echo - a simple CLI to send messages to a server", long_about = None)]
struct ClientCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
    /// The message to send
    message: String,
}
```

What is left is the `main` function, which will create a client and send a message to the server. As we use `async/await`, we need to use the `tokio` runtime. This is done by adding the `#[tokio::main]` attribute to our `main` function:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ClientCli::parse();

    let mut client = EchoServiceClient::connect(format!("http://{}:{}", cli.server, cli.port)).await?;

    let request = tonic::Request::new(EchoRequest {
        message: cli.message,
    });

    let response = client.echo(request).await?;

    println!("RESPONSE={:?}", response.into_inner().message);

    Ok(())
}
```

We define a `bin` target in our `Cargo.toml` file for our client as well:

```toml
[[bin]]
name = "echo-client"
path = "src/main.rs"
```

Start our client:

```bash
cargo run --bin echo-client -- "Hello World!"
```

You should see the following output:

```bash
RESPONSE="Hello World"
```

You can configure the server host and port using the `--server` and `--port` flags, similar to the server.

## Wrapping Up

In this article, we have learned how to use `tonic` and `clap` to create a simple gRPC microservice in `Rust` 🦀. We have also learned how to write a proto file and generate the client and server code using `tonic-build` via the `build.rs`

As always in tech, this kind of article can give you only a basic overview of the topic. Subjects like gRPC are much more complex and I highly recommend to continue reading the official documentation of `tonic` and `gRPC` to learn more about the topic.
