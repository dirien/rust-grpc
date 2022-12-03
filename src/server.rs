use tonic::{transport::Server, Request, Response, Status};

use api::echo_service_server::{EchoService, EchoServiceServer};
use api::{EchoRequest, EchoResponse};

use ::clap::{Parser};

pub mod api {
    tonic::include_proto!("api");
}

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
