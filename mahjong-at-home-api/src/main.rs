use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt, Interest},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    let ready = socket
        .ready(Interest::READABLE | Interest::WRITABLE)
        .await
        .unwrap();
    if ready.is_readable() {
        let mut buffer = [0; 1024];
        match socket.try_read(&mut buffer) {
            Ok(n) => {
                println!("read {} bytes", n);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                return;
            }
            Err(e) => {
                return;
            }
        }
        let str = String::from_utf8_lossy(&buffer[..]);
        println!("Received data: {}", str);
    }
    if ready.is_writable() {
        let message = "Hello, client!\n";
        match socket.try_write(message.as_bytes()) {
            Ok(n) => {
                println!("wrote {} bytes", n);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                return;
            }
            Err(e) => {
                return;
            }
        }
    }
}
