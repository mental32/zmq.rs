use tokio::net::TcpListener;
use tokio::prelude::*;

use std::convert::TryInto;
use zeromq::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Start server");
    let mut server = zeromq::bind(SocketType::REP, "127.0.0.1:5555").await?;

    loop {
        let mut socket = server.accept().await?;

        tokio::spawn(async move {
            loop {
                // TODO refactor me
                let mut repl: String = socket.recv().await.unwrap().try_into().unwrap();
                dbg!(&repl);
                repl.push_str(" Reply");
                socket.send(repl.into()).await.expect("Failed to send");
            }
        });
    }
}
