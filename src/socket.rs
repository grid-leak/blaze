use crate::packet::{Packet, PacketCodec};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::{Mutex, mpsc},
    task::JoinHandle,
};
use tokio_util::codec::Framed;

pub type BlazeRx = mpsc::UnboundedReceiver<Packet>;

#[derive(Clone)]
pub struct BlazeTx {
    tx: Arc<Mutex<mpsc::UnboundedSender<Packet>>>,
}

impl BlazeTx {
    fn new(tx: mpsc::UnboundedSender<Packet>) -> Self {
        Self {
            tx: Arc::new(Mutex::new(tx)),
        }
    }

    pub fn notify(&self, packet: Packet) {
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let guard = tx.lock().await;
            let _ = guard.send(packet);
        });
    }
}

pub fn spawn_socket<S>(stream: S) -> (JoinHandle<std::io::Result<()>>, BlazeRx, BlazeTx)
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let (inbound_tx, inbound_rx) = mpsc::unbounded_channel();
    let (outbound_tx, outbound_rx) = mpsc::unbounded_channel();
    let handle = tokio::spawn(run_socket(stream, inbound_tx, outbound_rx));

    let blaze_tx = BlazeTx::new(outbound_tx);

    (handle, inbound_rx, blaze_tx)
}

async fn run_socket<S>(
    stream: S,
    inbound_tx: mpsc::UnboundedSender<Packet>,
    mut outbound_rx: mpsc::UnboundedReceiver<Packet>,
) -> std::io::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut framed = Framed::new(stream, PacketCodec);

    loop {
        tokio::select! {
            // receive from socket -> forward inbound
            Some(result) = framed.next() => {
                let packet = result?;
                if inbound_tx.send(packet).is_err() {
                    break; // Receiver dropped
                }
            }

            // receive from channel -> send to socket
            Some(packet) = outbound_rx.recv() => {
                framed.send(packet).await?;
            }

            // both channels closed
            else => break,
        }
    }

    framed.close().await
}
