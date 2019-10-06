use std::borrow::{Borrow, BorrowMut};
use std::net::UdpSocket;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

#[derive(PartialOrd, PartialEq, Copy, Clone)]
enum State {
    Disconnected,
    Connecting,
    Connected,
    Quit,
}

#[derive(PartialOrd, PartialEq, Copy, Clone)]
enum Event {
    ConnectionRequest,
    ConnectionAcknowledge,
    Connected,
    Disconnected,
    Timeout,
    QuitRequest,
}

struct Tello {
    ip: String,
    port: u16,
    socket: Arc<UdpSocket>,
    sequence_number: u16,
    state: Arc<State>,
    receiving_thread: Option<JoinHandle<()>>,
}

impl Tello {
    fn new(ip: String, port: u16) -> Result<Tello, std::io::Error> {
        let socket = UdpSocket::bind("127.0.0.1:1111")?;
        socket.set_read_timeout(Some(std::time::Duration::from_secs(2)))?;
        socket.set_write_timeout(Some(std::time::Duration::from_secs(2)))?;

        let mut drone = Tello {
            ip,
            port,
            socket: Arc::new(socket),
            sequence_number: 0x01e4,
            state: Arc::new(State::Disconnected),
            receiving_thread: None,
        };

        return Result::Ok(drone);
    }

    pub fn connect(&mut self) {
        let socket = self.socket.clone();
        let state = self.state.clone();
        self.receiving_thread = Some(thread::spawn(move || {
            while *state.as_ref() != State::Quit {
                if *state.as_ref() == State::Connected {
                    // send stick command
                }

                let mut received_data: [u8; 2000] = [0; 2000];
                match socket.as_ref().recv(&mut received_data) {
                    Ok(length) => println!("data received"),
                    Err(error) => {
                        println!("receive errored {:?}", error);
                    }
                }
            }
        }));
    }

    fn state_machine(&mut self, event: Event) {
        let state = self.state.clone();

        match *state.as_ref() {
            State::Disconnected => {}
            State::Connecting => {}
            State::Connected => {}
            State::Quit => {}
        }
    }
}
