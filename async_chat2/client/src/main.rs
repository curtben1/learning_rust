use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    loop {
        println!("reading");
        let mut buffer = String::new();
        let inc = reader.read_line(&mut buffer).await.unwrap();
        if inc > 0 {
            println!("{inc}");
            println!("{buffer}");
            writer.write_all(b"hello\n").await?;
        } else {
            break;
        }
    }
    Ok(())
}