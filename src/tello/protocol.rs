const PACKET_START: u8 = 0xcc;

enum Commands {
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
    fn new();

    fn from_str(data: String) -> Self {
        Packet { buffer: data.into_bytes() }
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        Packet { buffer: bytes.into_vec() }
    }
}