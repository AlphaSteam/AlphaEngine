use crossbeam_channel::{Receiver, SendError, Sender};
use laminar::{Packet, Socket, SocketEvent};
use std::{net::SocketAddr, thread, time::Instant};

pub struct Net {
    packet_sender: Sender<laminar::Packet>,
    server_address: SocketAddr,
}

impl Net {
    pub fn connect(server_address: SocketAddr, client_address: SocketAddr) -> Self {
        println!(
            "Server address:{}. Client address:{}",
            server_address, client_address
        );
        let mut client = Socket::bind(client_address).unwrap();

        let event_receiver = client.get_event_receiver();
        let packet_sender = client.get_packet_sender();

        let _server_thread = thread::spawn(move || {
            client.start_polling();
            loop {
                client.manual_poll(Instant::now());
                let result = event_receiver.recv();

                match result {
                    Ok(socket_event) => match socket_event {
                        SocketEvent::Packet(packet) => {
                            let endpoint: SocketAddr = packet.addr();
                            let received_data: &[u8] = packet.payload();
                            println!("Endpoint: {}, received_data:{:?}", endpoint, received_data);
                        }
                        SocketEvent::Connect(connect_event) => {
                            println!("Client: {} has connected", connect_event)
                        }
                        SocketEvent::Timeout(timeout_event) => {
                            println!("Client: {} has timed_out", timeout_event);
                        }
                        SocketEvent::Disconnect(disconnect_event) => {
                            println!("Client: {} has disconnected", disconnect_event)
                        }
                    },
                    Err(e) => {
                        println!("Something went wrong when receiving, error: {:?}", e);
                    }
                }
            }
        });
        Net {
            packet_sender,
            server_address,
        }
    }
    pub fn send_packet(&self, payload: Vec<u8>) -> Result<(), SendError<laminar::Packet>> {
        let packet = Packet::reliable_unordered(self.server_address, payload);
        self.packet_sender.send(packet)
    }
}
