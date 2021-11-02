use tokio::sync::{oneshot, mpsc};
use tokio::time::{timeout, Duration};
use std::{thread, time};


async fn worker(chnl: oneshot::Sender<String>) {
    let ten_millis = time::Duration::from_millis(2000);

    thread::sleep(ten_millis);
    chnl.send("workert".to_string());
    
}

#[tokio::main]
async fn main() {


    loop{

        let (tx, mut rx) = mpsc::channel(32);
        let tx2 = tx.clone();

        tokio::spawn(async move {
            tx.send("sending from first handle").await;
        });
    
        tokio::spawn(async move {
            tx2.send("sending from second handle").await;
        });

        while let Some(message) = rx.recv().await {
            println!("GOT = {}", message);
    
        
        }
    
        let (worker_tx, worker_rx) = oneshot::channel();
        tokio::spawn(async move {
            worker(worker_tx).await;
        });
       
    
        // Wrap the future with a `Timeout` set to expire in 10 milliseconds.
        if let Err(_) = timeout(Duration::from_millis(1000), worker_rx).await {
            println!("did not receive value within 10 ms");
        }
    }

}