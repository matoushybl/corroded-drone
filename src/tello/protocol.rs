use crate::tello::crc::get_crc8;
use chrono::prelude::*;

const PACKET_START: u8 = 0xcc;

enum Command {
    WifiMsg = 0x001a,
    VideoEncoderRateCmd = 0x0020,
    VideoStartCmd = 0x0025,
    VideoRateQuery = 0x0028,
    TakePictureCommand = 0x0030,
    VideoModeCmd = 0x0031,
    ExposureCmd = 0x0034,
    LightMsg = 0x0035,
    TimeCmd = 0x0046,
    StickCmd = 0x0050,
    TakeoffCmd = 0x0054,
    LandCmd = 0x0055,
    FlightMsg = 0x0056,
    SetAltLimitCmd = 0x0058,
    FlipCmd = 0x005c,
    ThrowAndGoCmd = 0x005d,
    PalmLandCmd = 0x005e,
    TelloCmdFileSize = 0x0062,
    // pt50
    TelloCmdFileData = 0x0063,
    // pt50
    TelloCmdFileComplete = 0x0064,
    // pt48
    LogHeaderMsg = 0x1050,
    LogDataMsg = 0x1051,
    LogConfigMsg = 0x1052,
}

enum Flip {
    // FlipFront flips forward.
    FlipFront = 0,
    // FlipLeft flips left.
    FlipLeft = 1,
    // FlipBack flips backwards.
    FlipBack = 2,
    // FlipRight flips to the right.
    FlipRight = 3,
    // FlipForwardLeft flips forwards and to the left.
    FlipForwardLeft = 4,
    // FlipBackLeft flips backwards and to the left.
    FlipBackLeft = 5,
    // FlipBackRight flips backwards and to the right.
    FlipBackRight = 6,
    // FlipForwardRight flips forwards and to the right.
    FlipForwardRight = 7,
}

struct Packet {
    buffer: Vec<u8>
}

impl Packet {
    fn new(command: Command, packet_type: u8, data: Vec<u8>) -> Self {
        let raw_command = command as u8;
        let mut buffer: Vec<u8> = vec![
            PACKET_START, 0, 0, 0, packet_type, raw_command & 0xff, (raw_command >> 8) & 0xff, 0, 0,
        ];

        buffer.extend(data);

        Packet { buffer }
    }

    fn from_str(data: String) -> Self {
        Packet { buffer: data.into_bytes() }
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        Packet { buffer: bytes.to_vec() }
    }
}

struct CompletedPacket {
    buffer: Vec<u8>
}

impl Packet {
    fn complete(&mut self, sequence_number: u16) -> CompletedPacket {
        if self.buffer[0] != PACKET_START {
            return CompletedPacket { buffer: self.buffer.clone() };
        }
        let length = (self.buffer.len() + 2) as u16;

        self.buffer[1] = (length as u8 & 0xff) << 3;
        self.buffer[2] = (length >> 8) as u8 & 0xff;
        self.buffer[3] = get_crc8(&self.buffer[0..3]);
        self.buffer[7] = sequence_number as u8 & 0xff;
        self.buffer[8] = (sequence_number >> 8) as u8 & 0xff;

        return CompletedPacket { buffer: self.buffer.clone() };
    }

    fn add_u8(&mut self, value: u8) {
        self.buffer.push(value);
    }

    fn add_u16(&mut self, value: u16) {
        self.buffer.push(value as u8 & 0xff);
        self.buffer.push((value >> 8) as u8 & 0xff);
    }

    fn add_time(&mut self, time: DateTime<Utc>) {
        self.add_u16(time.hour() as u16);
        self.add_u16(time.minute() as u16);
        self.add_u16(time.second() as u16);
        self.add_u16(((time.nanosecond() / 1_000_000) as u16) & 0xff);
        self.add_u16((((time.nanosecond() / 1_000_000) as u16) >> 8) & 0xff);
    }

    fn get_time(&mut self) {
        // FIXME implement
    }
}
