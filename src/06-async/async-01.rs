extern crate tokio; // 1.5.0

use tokio::net::TcpStream;
use tokio::sync::mpsc;

#[allow(dead_code)]
type Request = ();

#[allow(unused_variables)]
#[allow(dead_code)]
async fn connect_to(addr: &str) -> mpsc::Sender<Request> {
    let (request_tx, mut request_rx) = mpsc::channel(32);
    
    let socket = TcpStream::connect(addr);
    
    tokio::spawn(async move {
        while let Some(_request) = request_rx.recv().await {
            // Elküldjük a requestet a socketen.
        }
    });
    
    request_tx
}

#[allow(dead_code)]
async fn use_socket() {
    let sender1 = connect_to("127.0.0.1:3000").await;
    let sender2 = sender1.clone();
    
    tokio::spawn(async move {
        let _ = sender1.send(()).await;
    });
    
    tokio::spawn(async move {
        let _ = sender2.send(()).await;
    });
}
