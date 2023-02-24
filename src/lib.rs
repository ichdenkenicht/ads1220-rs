


#[deny(missing_docs)]
extern crate embedded_hal as hal;




pub enum Command {
    RESET = 0b0000_0110,    //Resets the config registers to all zero
    START = 0b0000_1000,    //Starts a single conversion or the continous conversions
    POWERDOWN = 0b0000_0010,//Power down
    RDATA = 0b0001_0000,    //Loads the output shift register with the most recent result
    RREG = 0b0010_0000,     //0010_rrnn / rr -> address / nn -> number of bytes - 1
    WREG = 0b0100_0000,     //0100_rrnn / rr -> address / nn -> number of bytes - 1

}

impl Command {
    fn bits(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Register {
    CONFIG0 = 0x00,
    CONFIG1 = 0x01,
    CONFIG2 = 0x02,
    CONFIG3 = 0x03,
}

impl Register {
    fn addr(self) -> u8 {
        self as u8
    }
}

///Programmable Gain Amplifier (pga) ads1220 datasheet, p. 40
#[derive(Debug, Copy, Clone)]
pub enum PGA {
    Gain1 = 0b000,
    Gain2 = 0b001,
    Gain4 = 0b010,
    Gain8 = 0b011,
    Gain16 = 0b100,
    Gain32 = 0b101,
    Gain64 = 0b110,
    Gain128 = 0b111,
}

impl Default for PGA {
    fn default() -> Self {
        PGA::Gain1
    }
}

impl PGA {
    pub fn bits(self) -> u8 {
        self as u8
    }

    pub fn val(self) -> u8 {
        1 << self as u8
    }
}

//Channel
#[derive(Debug, Copy, Clone, Default)]
pub enum Channel {
    #[default]
    DIFF_AIN0_AIN1 = 0x0,
    DIFF_AIN0_AIN2 = 0x1,
    DIFF_AIN0_AIN3 = 0x2,
    DIFF_AIN1_AIN2 = 0x3,
    DIFF_AIN1_AIN3 = 0x4,
    DIFF_AIN2_AIN3 = 0x5,
    DIFF_AIN1_AIN0 = 0x6,
    DIFF_AIN3_AIN2 = 0x7,
    AIN0 = 0x8,
    AIN1 = 0x9,
    AIN2 = 0xA,
    AIN3 = 0xB,
    VREFDIFF4 = 0xC,
    AVDIFF4 = 0xD,
    VCC2 = 0xE,
    RESERVED = 0xF,
}

impl Channel {
    fn bits(self) -> u8 {
        self as u8
    }
}

//Data Rate Normal Mode
#[derive(Debug, Copy, Clone)]
pub enum DataRateNormal {
    SPS20 = 0x0,
    SPS45 = 0x1,
    SPS90 = 0x2,
    SPS175 = 0x3,
    SPS330 = 0x4,
    SPS600 = 0x5,
    SPS1000 = 0x6
}

impl DataRateNormal {
    fn bits(self) -> u8 {
        self as u8
    }
}

impl Default for DataRateNormal {
    fn default() -> Self {
        DataRateNormal::SPS20
    }
}

pub enum Mode {
    NORMAL = 0x0,
    DUTYCYCLE = 0x1,
    TURBO = 0x2,
}

impl Mode {
    fn bits(self) -> u8 {
        self as u8
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::NORMAL
    }
}

pub enum Reference {
    V2048 = 0x0,
    REF0 = 0x1,
    REF1 = 0x2,
    AV = 0x3,
}