use tokio::
{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};
use tokio::io::AsyncBufReadExt;
use tokio::sync::broadcast;

const IP_ADDR: &str = "127.0.0.1:7099";

#[tokio::main]
async fn main() {
    let server = TcpListener::bind(IP_ADDR).await.expect("wassup");
    let (tx, mut rx) = broadcast::channel(10);


    loop {
        let (mut sock, addr) = server.accept().await.expect("error in accpting conn");
        let mut rx = tx.subscribe();
        let tx = tx.clone();
        tokio::spawn(async move {
            let mut line = String::new();
            let (read, mut write) = sock.split();
            let mut read = BufReader::new(read);
            loop {
                tokio::select! {
                 res = read.read_line(&mut line) => {
                        if res.unwrap() ==0 {
                            break;
                        }
                        tx.send((String::from("vera vela illaya"),addr )).expect("could not send");
                        line.clear();
                    }

                    res = rx.recv() => {
                        let (msg, client_addr )= res.unwrap();
                        if addr != client_addr {
                            write.write_all("madaki naalu .... .....".as_bytes()).await.expect("eroor occurs while writing")
                        }
                    }

                }
            }
        });
    }
}

