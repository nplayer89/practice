use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

   loop { 
        match listener.accept().await {
            Ok((_socket, addr)) => println!("new client: {:?}", addr),
            Err(e) => println!("couldn't get client: {:?}", e),
        }
   }
}
