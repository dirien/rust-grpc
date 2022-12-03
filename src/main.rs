use api::echo_service_client::EchoServiceClient;
use api::EchoRequest;
use ::clap::{Parser};

pub mod api {
    tonic::include_proto!("api");
}

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
