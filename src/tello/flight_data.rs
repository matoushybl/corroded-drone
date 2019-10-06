use crate::tello::utils::*;

struct FlightData {
    battery_low: u8,
    battery_lower: u8,
    battery_percentage: u8,
    battery_state: u8,
    camera_state: u8,
    down_visual_state: u8,
    drone_battery_left: u16,
    drone_fly_time_left: u16,
    drone_hover: u8,
    em_open: u8,
    em_sky: u8,
    em_ground: u8,
    east_speed: i16,
    electrical_machinery_state: u8,
    factory_mode: u8,
    fly_mode: u8,
    fly_speed: u8,
    fly_time: u16,
    front_in: u8,
    front_lsc: u8,
    front_out: u8,
    gravity_state: u8,
    ground_speed: i16,
    height: u16,
    imu_calibration_state: u8,
    imu_state: u8,
    light_strength: u8,
    north_speed: i16,
    outage_recording: u8,
    power_state: u8,
    pressure_state: u8,
    smart_video_exit_mode: u8,
    temperature_height: u8,
    throw_fly_timer: u8,
    wifi_disturb: u8,
    wifi_strength: u8,
    wind_state: u8,
}

enum Error {
    NotEnoughData
}

impl FlightData {
    fn from_buffer(buffer: Vec<u8>) -> Result<Self, Error> {
        if buffer.len() < 24 {
            return Result::Err(Error::NotEnoughData);
        }

        let height = as_u16(&buffer[0..1]);
        let north_speed = as_i16(&buffer[2..3]);
        let east_speed = as_i16(&buffer[4..5]);
        let ground_speed = as_i16(&buffer[6..7]);
        let fly_time = as_u16(&buffer[8..9]);

        let imu_state = (buffer[10] >> 0) & 0x1;
        let pressure_state = (buffer[10] >> 1) & 0x1;
        let down_visual_state = (buffer[10] >> 2) & 0x1;
        let power_state = (buffer[10] >> 3) & 0x1;
        let battery_state = (buffer[10] >> 4) & 0x1;
        let gravity_state = (buffer[10] >> 5) & 0x1;
        let wind_state = (buffer[10] >> 7) & 0x1;

        let imu_calibration_state = buffer[11];
        let battery_percentage = buffer[12];
        let drone_battery_left = as_u16(&buffer[13..14]);
        let drone_fly_time_left = as_u16(&buffer[15..16]);

        let em_sky = (buffer[17] >> 0) & 0x1;
        let em_ground = (buffer[17] >> 1) & 0x1;
        let em_open = (buffer[17] >> 2) & 0x1;
        let drone_hover = (buffer[17] >> 3) & 0x1;
        let outage_recording = (buffer[17] >> 4) & 0x1;
        let battery_low = (buffer[17] >> 5) & 0x1;
        let battery_lower = (buffer[17] >> 6) & 0x1;
        let factory_mode = (buffer[17] >> 7) & 0x1;

        let fly_mode = buffer[18];
        let throw_fly_timer = buffer[19];
        let camera_state = buffer[20];
        let electrical_machinery_state = buffer[21];

        let front_in = (buffer[22] >> 0) & 0x1;
        let front_out = (buffer[22] >> 1) & 0x1;
        let front_lsc = (buffer[22] >> 2) & 0x1;

        let temperature_height = (buffer[23] >> 0) & 0x1;

        return Result::Ok(FlightData {
            battery_low,
            battery_lower,
            battery_percentage,
            battery_state,
            camera_state,
            down_visual_state,
            drone_battery_left,
            drone_fly_time_left,
            drone_hover,
            em_open,
            em_sky,
            em_ground,
            east_speed,
            electrical_machinery_state,
            factory_mode,
            fly_mode,
            fly_speed: 0,
            fly_time,
            front_in,
            front_lsc,
            front_out,
            gravity_state,
            ground_speed,
            height,
            imu_calibration_state,
            imu_state,
            light_strength: 0,
            north_speed,
            outage_recording,
            power_state,
            pressure_state,
            smart_video_exit_mode: 0,
            temperature_height,
            throw_fly_timer,
            wifi_disturb: 0,
            wifi_strength: 0,
            wind_state,
        });
    }
}

use core::fmt::Formatter;

impl core::fmt::Display for FlightData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "ALT: {} | SPD: {} | BAT: {} | WIFI: {} | CAM: {} | MODE: {}",
               self.height, self.ground_speed, self.battery_percentage, self.wifi_strength,
               self.camera_state, self.fly_mode)
    }
}