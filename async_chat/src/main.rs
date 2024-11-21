use std::io;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::task::JoinSet;

async fn process_socket(mut socket: TcpStream, addr: SocketAddr) -> String {
    println!("accepted connection from {}", addr);
    socket.write_all(b"hello client").await.unwrap();
    let (reader, mut writer) = socket.into_split();
    let mut last_msg = "".to_string();
    tokio::spawn(async move {
        last_msg = read_from_socket(reader).await.unwrap();
    });
    print!("about tow ");
    tokio::spawn(async move {
        loop {
            print!("Message: ");
            let mut msg = String::new();
            io::stdin().read_line(&mut msg).expect("error");
            if msg == "exit\n" {
                break;
            }
            write_to_socket(&mut writer, msg).await;
        }
    });

    "temp".to_string()
}

async fn write_to_socket(mut writer: &mut tokio::net::tcp::OwnedWriteHalf, msg: String) {
    writer.write_all(msg.as_bytes()).await.unwrap();
}

async fn read_from_socket(reader: OwnedReadHalf) -> Option<String> {
    println!("Reading");
    let mut reader = BufReader::new(reader);
    let mut final_message: String = "".to_string();
    loop {
        let mut buffer = String::new();
        let inc = reader.read_line(&mut buffer).await.unwrap();

        if inc > 0 {
            println!("recieved: {buffer}");
            final_message = buffer
        } else {
            return Some(final_message);
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let mut join_set: JoinSet<String> = JoinSet::new();
    loop {
        println!("Listening...");
        let (socket, addr) = listener.accept().await?;
        join_set.spawn(async move {
            process_socket(socket, addr).await
        });
        if join_set.len() > 2 {
            break;
        }
    }
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(output) => println!("Task completed with result: {:?}", output),
            Err(e) => println!("Task failed: {}", e),
        }
    }
    println!("exited");
    Ok(())
}
