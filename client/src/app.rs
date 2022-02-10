use std::time::Duration;

cfg_if! {
    if #[cfg(feature = "mquad")] {
        use miniquad::info;
    } else {
        use log::info;
    }
}

use naia_client_socket::{Packet, PacketReceiver, PacketSender, Socket, Timer};

use naia_socket_docker_example_shared::{get_shared_config, PING_MSG, PONG_MSG};

pub struct App {
    packet_sender: PacketSender,
    packet_receiver: PacketReceiver,
    message_count: u8,
    timer: Timer,
}

impl App {
    pub fn new() -> App {
        info!("Naia Client Socket Demo started");

        let shared_config = get_shared_config();

        let mut socket = Socket::new(shared_config);
        socket.connect("http://192.168.1.7:14191");

        App {
            packet_sender: socket.get_packet_sender(),
            packet_receiver: socket.get_packet_receiver(),
            message_count: 0,
            timer: Timer::new(Duration::from_secs(1)),
        }
    }

    pub fn update(&mut self) {
        match self.packet_receiver.receive() {
            Ok(event) => match event {
                Some(packet) => {
                    let message = String::from_utf8_lossy(packet.payload());
                    info!("Client recv <- {}: {}", self.packet_receiver.remote_addr(), message);

                    if message.eq(PONG_MSG) {
                        self.message_count += 1;
                    }
                }
                None => {
                    if self.timer.ringing() {
                        self.timer.reset();
                        if self.message_count < 10 {
                            let to_server_message: String = PING_MSG.to_string();
                            info!("Client send -> {}: {}", self.packet_sender.remote_addr(), to_server_message);
                            self.packet_sender
                                .send(Packet::new(to_server_message.into_bytes()));
                        }
                    }
                }
            },
            Err(err) => {
                info!("Client Error: {}", err);
            }
        }
    }
}
