//! Register abstractions.
//!
//! Register abstractions for the 4 config registers of the ADS1220 ADC from TI
//!
//! https://www.ti.com/lit/ds/symlink/ads1220.pdf


///Register 0-3 Bitflags
pub struct BitFlags;
#[allow(unused)]
impl BitFlags {
    //Reg 0
    ///PGA_BYPASS bitflag
    pub const PB: u8 = 0b0000_0001; 
        
    //Reg 1
    ///Burn-Out current source bitflag
    pub const BCS: u8 = 0b0000_0001;  
    
    ///Temperatur Sensor Mode bitflag
    pub const TS: u8 = 0b0000_0010; 
       
    ///Conversion Mode Single or Continous bitflag
    pub const CM: u8 = 0b0000_0100;     
    //Reg 2
    ///Low side switch bitflag
    pub const PSW: u8 = 0b0000_1000;    
    //Reg 3
    ///Data ready mode bitflag
    pub const DRDYM: u8 = 0b0000_0010;  
}


#[derive(Debug, Copy, Clone)]
pub enum Register {
    CONFIG0 = 0x00,
    CONFIG1 = 0x01,
    CONFIG2 = 0x02,
    CONFIG3 = 0x03,
}

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }
}


//Reg0


#[derive(Debug, Copy, Clone, Default)]
pub enum PGA_BYPASS {
    #[default]
    ENABLED = 0,
    DISABLED = 1,
}

///Programmable Gain Amplifier (pga) ads1220 datasheet, p. 40
#[derive(Debug, Copy, Clone, Default)]
pub enum PGA {
    #[default]
    Gain1 = 0b0000_0000,
    Gain2 = 0b0000_0001,
    Gain4 = 0b0000_0010,
    Gain8 = 0b0000_0011,
    Gain16 = 0b0000_0100,
    Gain32 = 0b0000_0101,
    Gain64 = 0b0000_0110,
    Gain128 = 0b0000_0111,
}

impl PGA {
    pub fn bits(self) -> u8 {
        self as u8
    }

    pub fn bits_on_pos(self) -> u8 {
        (self as u8) << 1
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
    pub fn bits(self) -> u8 {
        self as u8
    }
    pub fn bits_on_pos(self) -> u8 {
        (self as u8) << 4
    }
}

//Reg 1

///Burn-our current source
#[derive(Debug, Copy, Clone, Default)]
pub enum BCS {
    #[default]
    OFF = 0,
    ON = 1,
}

///Temperatur sensor mode
#[derive(Debug, Copy, Clone, Default)]
pub enum TS {
    #[default]
    DISABLED = 0,
    ENABLED = 1,
}

///Conversion Mode
#[derive(Debug, Copy, Clone, Default)]
pub enum CM {
    #[default]
    SINGLE = 0,
    CONTINUOUS = 1,
}

///Operation Mode
#[derive(Debug, Copy, Clone, Default)]
pub enum Mode {
    #[default]
    NORMAL = 0x0,
    DUTYCYCLE = 0x1,
    TURBO = 0x2,
}

impl Mode {
    pub fn bits(self) -> u8 {
        self as u8
    }
    pub fn bits_on_pos(self) -> u8 {
        (self as u8) << 3
    }
}

///Data Rate Normal Mode
#[derive(Debug, Copy, Clone, Default)]
pub enum DataRate {
    #[default]
    SPS20 = 0x0,
    SPS45 = 0x1,
    SPS90 = 0x2,
    SPS175 = 0x3,
    SPS330 = 0x4,
    SPS600 = 0x5,
    SPS1000 = 0x6
}

impl DataRate {
    pub fn bits(self) -> u8 {
        self as u8
    }
    pub fn bits_on_pos(self) -> u8 {
        (self as u8) << 5
    }
}

//Reg 2

#[derive(Debug, Copy, Clone, Default)]
pub enum Idac {
    #[default]
    OFF = 0x0,
    U10 = 0x1,
    U50 = 0x2,
    U100 = 0x3,
    U250 = 0x4,
    U500 = 0x5,
    U1000 = 0x6,
    U1500 = 0x7,
}

impl Idac {
    pub fn bits(self) -> u8 {
        self as u8
    }
}

///Low-side power switch config
#[derive(Debug, Copy, Clone, Default)]
pub enum PSW {
    #[default]
    OPEN = 0,
    CLOSING = 1,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Filter {
    #[default]
    NONE = 0x0,
    HZ5060 = 0x1,
    HZ50 = 0x2,
    HZ60 = 0x3,
}

impl Filter {
    pub fn bits(self) -> u8 {
        self as u8
    }
    pub fn bits_on_pos(self) -> u8 {
        (self as u8) << 4
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Reference {
    #[default]
    V2048 = 0x0,
    REF0 = 0x1,
    REF1 = 0x2,
    AV = 0x3,
}

impl Reference {
    pub fn bits(self) -> u8 {
        self as u8
    }
    pub fn bits_on_pos(self) -> u8 {
        (self as u8) << 6
    }
}



//Reg 3


///Data Ready Mode
#[derive(Debug, Copy, Clone, Default)]
pub enum DRDYM {
    #[default]
    DRDY = 0,
    DOUT_DRDY = 1,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Idac1r {
    #[default]
    OFF = 0x0,
    AIN0 = 0x1,
    AIN1 = 0x2,
    AIN2 = 0x3,
    AIN3 = 0x4,
    REFP0 = 0x5,
    REFN0 = 0x6,
}

impl Idac1r {
    pub fn bits(self) -> u8 {
        self as u8
    }
    pub fn bits_on_pos(self) -> u8 {
        (self as u8) << 2
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Idac2r {
    #[default]
    OFF = 0x0,
    AIN0 = 0x1,
    AIN1 = 0x2,
    AIN2 = 0x3,
    AIN3 = 0x4,
    REFP0 = 0x5,
    REFN0 = 0x6,
}

impl Idac2r {
    pub fn bits(self) -> u8 {
        self as u8
    }
    pub fn bits_on_pos(self) -> u8 {
        (self as u8) << 5
    }
}
