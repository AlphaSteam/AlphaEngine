use core::time;
use crossbeam_channel::Sender;
pub use laminar::{Config, Packet, Socket, SocketEvent};
use std::{net::SocketAddr, thread, time::Instant};
pub struct Net {
    packet_sender: Sender<laminar::Packet>,
    server_address: SocketAddr,
}

impl Net {
    fn connection(mut client: Socket, server_address: SocketAddr) -> Self {
        let packet_sender = client.get_packet_sender();

        let _server_thread = thread::spawn(move || {
            loop {
                client.manual_poll(Instant::now());
                let result = client.recv();
                match result {
                    Some(socket_event) => match socket_event {
                        SocketEvent::Packet(packet) => {
                            let endpoint: SocketAddr = packet.addr();
                            let received_data: &[u8] = packet.payload();
                            println!(
                                "Endpoint: {}, received_data:{}",
                                endpoint,
                                std::str::from_utf8(received_data).unwrap()
                            );
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
                    None => {
                        //println!("Something went wrong when receiving");
                    }
                }
            }
        });
        Net {
            packet_sender,
            server_address,
        }
    }
    pub fn connect(server_address: SocketAddr, client_address: SocketAddr) -> Self {
        println!(
            "Server address:{}. Client address:{}",
            server_address, client_address
        );
        let client = Socket::bind(client_address).unwrap();
        Net::connection(client, server_address)
    }
    pub fn connect_with_config(
        server_address: SocketAddr,
        client_address: SocketAddr,
        config: Config,
    ) -> Self {
        println!(
            "Server address:{}. Client address:{}",
            server_address, client_address
        );
        let client = Socket::bind_with_config(client_address, config).unwrap();
        Net::connection(client, server_address)
    }
    pub fn send_packet(&self, payload: Vec<u8>) {
        let packet_sender = self.packet_sender.clone();
        let server_address = self.server_address;
        let _thread = thread::spawn(move || {
            let packet = Packet::reliable_unordered(server_address, payload);
            println!("Packet sent: {:?}", packet);
            packet_sender.send(packet)
        });
    }
    pub fn test_packets(&self) {
        let packet_sender = self.packet_sender.clone();
        let server_address = self.server_address;
        let _thread = thread::spawn(move || {
            let mut i: i32 = 0;
            loop {
                i += 1;
                let packet = Packet::reliable_unordered(server_address, i.to_string().into_bytes());

                println!(
                    "Packet sent: {:?} Data: {}",
                    packet,
                    std::str::from_utf8(packet.payload()).unwrap()
                );
                let _result = packet_sender.send(packet);
                thread::sleep(time::Duration::from_secs(1));
            }
        });
    }
}
