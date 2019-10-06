extern crate corroded_drone;
use std::time::Duration;

fn main() {
    let mut drone = corroded_drone::tello::Tello::new().unwrap();

    drone.connect();

    drone.take_off();
    std::thread::sleep(Duration::from_secs(2));
    drone.land();
}
