use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

#[derive(Copy, Clone)]
pub struct State {
    pub roll: i16,
    pub pitch: i16,
    pub yaw: i16,
    pub ground_velocity_x: i16,
    pub ground_velocity_y: i16,
    pub ground_velocity_z: i16,
    pub temperature_minimum: u8,
    pub temperature_maximum: u8,
    pub tof_value: i16,
    pub height: i16,
    pub battery_percentage: u8,
    pub barometer_height: f32,
    pub time: u16,
    pub ground_acceleration_x: f32,
    pub ground_acceleration_y: f32,
    pub ground_acceleration_z: f32,
}

impl State {
    fn new() -> Self {
        State {
            roll: 0,
            pitch: 0,
            yaw: 0,
            ground_velocity_x: 0,
            ground_velocity_y: 0,
            ground_velocity_z: 0,
            temperature_minimum: 0,
            temperature_maximum: 0,
            tof_value: 0,
            height: 0,
            battery_percentage: 0,
            barometer_height: 0.0,
            time: 0,
            ground_acceleration_x: 0.0,
            ground_acceleration_y: 0.0,
            ground_acceleration_z: 0.0,
        }
    }
}

pub struct Tello {
    command_socket: Arc<Mutex<UdpSocket>>,
    state_socket: Arc<Mutex<UdpSocket>>,
    command_thread: Option<JoinHandle<()>>,
    state_thread: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
    state: Arc<Mutex<State>>,
    drone_acked: Arc<AtomicBool>,
}

pub enum Flip {
    Left,
    Right,
    Forward,
    Backward,
}

#[derive(Debug)]
pub enum TelloError {
    IO(std::io::Error),
    AckNotReceived
}

impl Flip {
    fn value(self) -> char {
        match self {
            Flip::Left => 'l',
            Flip::Right => 'r',
            Flip::Forward => 'f',
            Flip::Backward => 'b',
        }
    }
}

impl Tello {
    pub fn new() -> Result<Self, std::io::Error> {
        let command_socket = UdpSocket::bind("0.0.0.0:9009")?;
        let state_socket = UdpSocket::bind("0.0.0.0:8890")?;

        command_socket.set_nonblocking(true)?;
        state_socket.set_nonblocking(true)?;

        Ok(Tello {
            command_socket: Arc::new(Mutex::new(command_socket)),
            state_socket: Arc::new(Mutex::new(state_socket)),
            command_thread: None,
            state_thread: None,
            running: Arc::new(AtomicBool::new(false)),
            state: Arc::new(Mutex::new(State::new())),
            drone_acked: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn connect(&mut self) -> Result<usize, TelloError> {
        self.running.store(true, Ordering::SeqCst);

        let command_running = self.running.clone();
        let command_socket = self.command_socket.clone();
        let command_acked = self.drone_acked.clone();
        self.command_thread = Some(std::thread::spawn(move || {
            while command_running.load(Ordering::SeqCst) {
                let mut buffer: [u8; 1500] = [0; 1500];
                let socket = command_socket.lock().unwrap();
                let result = socket.recv_from(&mut buffer);
                match result {
                    Ok((size, _)) => {
                        let response = std::str::from_utf8(&buffer[..size])
                            .unwrap_or_default()
                            .trim();
                        if response == "ok" {
                            command_acked.store(true, Ordering::SeqCst);
                        }
                    }
                    Err(_) => { }
                }

                std::thread::sleep(Duration::from_millis(10));
            }
        }));

        let state_running = self.running.clone();
        let state_socket = self.state_socket.clone();
        let state = self.state.clone();
        self.state_thread = Some(std::thread::spawn(move || {
            while state_running.load(Ordering::SeqCst) {
                let mut buffer: [u8; 1500] = [0; 1500];
                let socket = state_socket.lock().unwrap();

                match socket.recv(&mut buffer) {
                    Ok(size) => {
                        let string = std::str::from_utf8(&buffer[..size - 1])
                            .unwrap_or_default()
                            .trim();
                        let parts: Vec<&str> = string.split(";").collect();

                        let mut parameter_map: HashMap<&str, &str> = HashMap::new();
                        for parameter in &parts {
                            if parameter.len() <= 1 || !parameter.contains(':') {
                                continue;
                            }
                            let parameter_parts: Vec<&str> = (*parameter).split(":").collect();

                            parameter_map.insert(parameter_parts[0], parameter_parts[1]);
                        }

                        let mut state = state.lock().unwrap();
                        state.roll = parameter_map["roll"].parse().unwrap();
                        state.pitch = parameter_map["pitch"].parse().unwrap();
                        state.yaw = parameter_map["yaw"].parse().unwrap();
                        state.ground_velocity_x = parameter_map["vgx"].parse().unwrap();
                        state.ground_acceleration_y = parameter_map["vgy"].parse().unwrap();
                        state.ground_velocity_z = parameter_map["vgz"].parse().unwrap();
                        state.temperature_minimum = parameter_map["templ"].parse().unwrap();
                        state.temperature_maximum = parameter_map["temph"].parse().unwrap();
                        state.tof_value = parameter_map["tof"].parse().unwrap();
                        state.height = parameter_map["h"].parse().unwrap();
                        state.battery_percentage = parameter_map["bat"].parse().unwrap();
                        state.barometer_height = parameter_map["baro"].parse().unwrap();
                        state.time = parameter_map["time"].parse().unwrap();
                        state.ground_acceleration_x = parameter_map["agx"].parse().unwrap();
                        state.ground_acceleration_y = parameter_map["agy"].parse().unwrap();
                        state.ground_acceleration_z = parameter_map["agz"].parse().unwrap();
                    }
                    Err(_) => {}
                }

                std::thread::sleep(Duration::from_millis(50));
            }
        }));

        self.send_command("command", true)
    }

    pub fn disconnect(&mut self) {
        self.running.store(true, Ordering::SeqCst);
        self.state_thread = None;
        self.command_thread = None;
    }

    pub fn send_command(&self, command: &str, acked: bool) -> Result<usize, TelloError> {
        {
            let mutex = self.command_socket.clone();
            let socket = mutex.lock().unwrap();
            socket.send_to(command.as_bytes(), "192.168.10.1:8889").map_err(|err| TelloError::IO(err))?;
        }

        if !acked {
            return Ok(command.len());
        }

        let sending_acked = self.drone_acked.clone();
        let now = Instant::now();
        while now.elapsed() < Duration::from_millis(10000) {
            if sending_acked.load(Ordering::SeqCst) {
                sending_acked.store(false, Ordering::SeqCst);
                return Ok(command.len());
            }

            std::thread::sleep(Duration::from_millis(50));
        }
        Err(TelloError::AckNotReceived)
    }

    pub fn get_state(&self) -> State {
        return self.state.clone().lock().unwrap().clone();
    }

    pub fn take_off(&self) -> Result<usize, TelloError> {
        self.send_command("takeoff", false)
    }

    pub fn land(&self) -> Result<usize, TelloError> {
        self.send_command("land", false)
    }

    pub fn stream_on(&self) -> Result<usize, TelloError> {
        self.send_command("streamon", true)
    }

    pub fn stream_off(&self) -> Result<usize, TelloError> {
        self.send_command("streamoff", true)
    }

    pub fn emergency(&self) -> Result<usize, TelloError> {
        self.send_command("emergency", true)
    }

    pub fn up(&self, distance_cm: u16) -> Result<usize, TelloError> {
        self.send_command(format!("up {}", distance_cm).as_str(), true)
    }

    pub fn down(&self, distance_cm: u16) -> Result<usize, TelloError> {
        self.send_command(format!("down {}", distance_cm).as_str(), true)
    }

    pub fn left(&self, distance_cm: u16) -> Result<usize, TelloError> {
        self.send_command(format!("left {}", distance_cm).as_str(), true)
    }

    pub fn right(&self, distance_cm: u16) -> Result<usize, TelloError> {
        self.send_command(format!("right {}", distance_cm).as_str(), true)
    }

    pub fn forward(&self, distance_cm: u16) -> Result<usize, TelloError> {
        self.send_command(format!("forward {}", distance_cm).as_str(), true)
    }

    pub fn back(&self, distance_cm: u16) -> Result<usize, TelloError> {
        self.send_command(format!("back {}", distance_cm).as_str(), true)
    }

    pub fn cw(&self, angle_millidegs: u16) -> Result<usize, TelloError> {
        self.send_command(format!("cw {}", angle_millidegs).as_str(), true)
    }

    pub fn ccw(&self, angle_millidegs: u16) -> Result<usize, TelloError> {
        self.send_command(format!("ccw {}", angle_millidegs).as_str(), true)
    }

    pub fn flip(&self, flip: Flip) -> Result<usize, TelloError> {
        self.send_command(format!("flip {}", flip.value()).as_str(), true)
    }

    pub fn go(&self, x_cm: u16, y_cm: u16, z_cm: u16, speed: u8) -> Result<usize, TelloError> {
        self.send_command(format!("go {} {} {} {}", x_cm, y_cm, z_cm, speed).as_str(), true)
    }

    pub fn curve(
        &self,
        x1_cm: u16,
        y1_cm: u16,
        z1_cm: u16,
        x2_cm: u16,
        y2_cm: u16,
        z2_cm: u16,
        speed: u8,
    ) -> Result<usize, TelloError> {
        self.send_command(
            format!(
                "curve {} {} {} {} {} {} {}",
                x1_cm, y1_cm, z1_cm, x2_cm, y2_cm, z2_cm, speed
            )
            .as_str(), true
        )
    }

    pub fn rc(
        &self,
        left_right: i8,
        forward_backward: i8,
        up_down: i8,
        yaw: i8,
    ) -> Result<usize, TelloError> {
        self.send_command(
            format!("rc {} {} {} {}", left_right, forward_backward, up_down, yaw).as_str(), false
        )
    }

    pub fn speed(&self, speed_cms: u8) -> Result<usize, TelloError> {
        self.send_command(format!("speed {}", speed_cms).as_str(), true)
    }

    pub fn wifi(&self, ssid: &str, password: &str) -> Result<usize, TelloError> {
        self.send_command(format!("wifi {} {}", ssid, password).as_str(), true)
    }
}
