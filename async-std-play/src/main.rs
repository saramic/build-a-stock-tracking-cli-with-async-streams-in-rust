// use async_std::prelude::*;
use async_std::net::UdpSocket;

async fn say_hello() {
    println!("Hello, world!");
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    say_hello().await;

    // let a = async { 1u8 };
    // let b = async { 2u8 };
    // assert_eq!(a.join(b).await, (1u8, 2u8));

    // can test with
    //      nc -v -u 127.0.0.1 8080
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    println!("Listening on {}", socket.local_addr()?);

    let mut buf = vec![0u8; 1024];

    loop {
        let (recv, peer) = socket.recv_from(&mut buf).await?;
        let sent = socket.send_to(&buf[..recv], &peer).await?;
        println!("Sent {} out of {} bytes to {}", sent, recv, peer);
    }
}