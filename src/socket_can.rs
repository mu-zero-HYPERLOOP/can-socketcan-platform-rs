use std::time::Instant;

use canzero_common::{NetworkFrame, TNetworkFrame};
use color_print::cprintln;
use tokio::sync::mpsc;

use crate::can_socket::CanSocket;

#[derive(Debug)]
pub struct SocketCan {
    sockets: Vec<CanSocket>,
    rx: tokio::sync::Mutex<mpsc::Receiver<TNetworkFrame>>,
}

impl SocketCan {
    pub async fn connect(buses: &[&str]) -> std::io::Result<Self> {
        let mut sockets : Vec<CanSocket> = vec![];
        for bus in buses {
            sockets.push(CanSocket::open(bus)?);
        }

        let (tx, rx) = mpsc::channel(16);

        let rx_sockets = sockets.clone();

        let start_of_run = Instant::now();

        for (bus_id, socket) in rx_sockets.into_iter().enumerate() {
            let tx = tx.clone();
            tokio::task::spawn_blocking(move || loop {
                match socket.receive() {
                    Ok(frame) => {
                        let tx = tx.clone();
                        tokio::spawn(async move {
                            let frame = match frame {
                                Ok(can_frame) => NetworkFrame {
                                    bus_id: bus_id as u32,
                                    can_frame,
                                },
                                Err(_) => todo!(),
                            };
                            let tframe = TNetworkFrame::now(start_of_run, frame);

                            if let Err(_) = tx.send(tframe).await {
                                cprintln!("<red>Failed to send on SocketCan [bus_id = {bus_id:?}]</red>");
                            }
                        });
                    }
                    Err(_) => {
                        cprintln!("<red>Failed to receive from SocketCAN [bus_id = {bus_id:?}]</red>")
                    }
                }
            });
        }
        Ok(Self {
            sockets,
            rx: tokio::sync::Mutex::new(rx),
        })
    }
    pub async fn send(&self, frame: &TNetworkFrame) -> std::io::Result<()> {
        self.sockets[frame.value.bus_id as usize]
            .transmit(&frame.value.can_frame)
    }
    pub async fn recv(&self) -> Option<TNetworkFrame> {
        self.rx.lock().await.recv().await
    }
}
