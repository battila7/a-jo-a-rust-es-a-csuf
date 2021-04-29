extern crate tokio; // 1.5.0

use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[allow(dead_code)]
type Payload = ();
type Reply = ();
type Request = (Payload, oneshot::Sender<Reply>);

#[allow(unused_variables)]
#[allow(dead_code)]
async fn connect_to(addr: &str) -> mpsc::Sender<Request> {
    let (request_tx, mut request_rx): (mpsc::Sender<Request>, mpsc::Receiver<Request>) 
        = mpsc::channel(32);
    
    let socket = TcpStream::connect(addr);
    
    tokio::spawn(async move {
        while let Some((payload, reply_tx)) = request_rx.recv().await {
            // Elküldjük a payloadot a socketen.
            
            // Visszaküldjük a sockettől kapott választ.
            let _ = reply_tx.send(());
        }
    });
    
    request_tx
}

#[allow(dead_code)]
async fn use_socket() {
    let sender1 = connect_to("127.0.0.1:3000").await;
    let sender2 = sender1.clone();
    
    tokio::spawn(async move {
        let (reply_tx, reply_rx) = oneshot::channel();
        
        let _ = sender1.send(((), reply_tx)).await;
        
        let _reply = reply_rx.await;
    });
    
    tokio::spawn(async move {
        let (reply_tx, reply_rx) = oneshot::channel();
        
        let _ = sender2.send(((), reply_tx)).await;
        
        let _reply = reply_rx.await;
    });
}
