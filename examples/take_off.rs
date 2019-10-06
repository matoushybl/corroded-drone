extern crate corroded_drone;

#[macro_use]
extern crate log;

use std::time::Duration;

fn main() {
    let mut drone = corroded_drone::tello::Tello::new().unwrap();

    drone.connect();

    drone.take_off();
    std::thread::sleep(Duration::from_secs(10));
    drone.land();
}
