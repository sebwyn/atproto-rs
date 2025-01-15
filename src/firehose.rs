
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use tokio::{self, io::{stdout, AsyncWriteExt}};

pub struct Firehose {
}

impl Firehose {
    pub async fn new(relay_provider: &str) {
        // let (mut socket, response) = connect();let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
        // tokio::spawn(read_stdin(stdin_tx));
       
        let connection_url = format!("{}/xrpc/com.atproto.sync.subscribeRepos", relay_provider);

        let (ws_stream, _) = connect_async(&connection_url).await.expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");

        let (_write, read) = ws_stream.split();

        // let stdin_to_ws = stdin_rx.map(Ok).forward(write);
        let ws_to_stdout = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
                stdout().write_all(&data).await.unwrap();
            })
        };
        ws_to_stdout.await; 
    }
}
