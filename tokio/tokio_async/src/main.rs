use tokio::net::TcpListener;
use std::io::Result;

//hello
#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    match listener.accept().await {
        Ok((_socket, addr)) => println!("new client: {:?}", addr),
        Err(e) => println!("couldn't get client: {:?}", e),
    }
    
    Ok(())
}
