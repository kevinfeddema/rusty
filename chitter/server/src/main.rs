use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    const PORT: &str = "localhost:5000";
    const MSG_SIZE: usize = 1024;

    let listener = TcpListener::bind(PORT)
        .await
        .expect("Failed to bind TCP listener to port");

    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener
            .accept()
            .await
            .expect("Failed to accept incoming connection");

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader
                    .read_line(&mut line) => {
                        if result.expect("Failed to read from socket") == 0 {
                            break;
                        }
                        tx.send((line.clone(), addr)).expect("Failed to send message");
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.expect("Failed to receive message");

                        if addr != other_addr {
                            writer
                            .write_all(msg.as_bytes())
                            .await
                            .expect("Failed to write to socket");
                        }

                    }
                }
            }
        });
    }
}
