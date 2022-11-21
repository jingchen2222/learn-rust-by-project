use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::spawn;
use tokio::sync::broadcast;

/// tcp echo server
/// tcp client

#[tokio::main]
async fn main() {
    println!("Welcome to online chat stream!");
    let mut listener = TcpListener::bind("localhost:7373").await.unwrap();
    let (tx, mut rx) = broadcast::channel(20);

    loop {
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let (mut socket, addr) = listener.accept().await.unwrap();
        spawn(async move {
            let (read, mut writer) = socket.split();
            let mut reader = BufReader::new(read);
            let mut line = String::new();
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if (result.unwrap() == 0) {
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }

                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        if (addr != other_addr) {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
