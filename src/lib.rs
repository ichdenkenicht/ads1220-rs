


#[deny(missing_docs)]
extern crate embedded_hal as hal;

use hal::spi::blocking::{SpiBus, SpiBusRead, SpiBusWrite, SpiDevice};


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
#[derive(Debug, Copy, Clone, Default)]
pub enum PGA {
    #[default]
    Gain1 = 0b000,
    Gain2 = 0b001,
    Gain4 = 0b010,
    Gain8 = 0b011,
    Gain16 = 0b100,
    Gain32 = 0b101,
    Gain64 = 0b110,
    Gain128 = 0b111,
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
#[derive(Debug, Copy, Clone, Default)]
pub enum DataRateNormal {
    #[default]
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

#[derive(Debug, Copy, Clone, Default)]
pub enum Mode {
    #[default]
    NORMAL = 0x0,
    DUTYCYCLE = 0x1,
    TURBO = 0x2,
}

impl Mode {
    fn bits(self) -> u8 {
        self as u8
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
    fn bits(self) -> u8 {
        self as u8
    }
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
    fn bits(self) -> u8 {
        self as u8
    }
}

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
    fn bits(self) -> u8 {
        self as u8
    }
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
    fn bits(self) -> u8 {
        self as u8
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
    fn bits(self) -> u8 {
        self as u8
    }
}


#[derive(Debug, Copy, Clone, Default)]
pub struct Config {
    cfg0: u8,
    cfg1: u8,
    cfg2: u8,
    cfg3: u8,
}



pub struct ADS1220<SPI> {
    spi: SPI,
    config: Config,
}

impl<SPI> ADS1220<SPI>
where
     SPI: SpiDevice,
     SPI::Bus: SpiBus, // or SpiBusRead/SpiBusWrite if you only need to read or only write.
{
     pub fn new(spi: SPI) -> Self {
         Self { spi, config: Config::default() }
     }

    pub fn read_foo(&mut self) -> Result<[u8; 2], MyError<SPI::Error>> {
        let mut buf = [0; 2];

         // `transaction` asserts and deasserts CS for us. No need to do it manually!
        self.spi.transaction(|bus| {
             bus.write(&[0x90])?;
             bus.read(&mut buf)
        }).map_err(MyError::Spi)?;

        Ok(buf)
    }

    pub fn read_register(&mut self, reg: Register) -> Result<u8, MyError<SPI::Error>> {
        let mut res: [u8; 1] = [0];

        // `transaction` asserts and deasserts CS for us. No need to do it manually!
        self.spi.transaction(|bus| {
             bus.write(&[Command::RREG.bits() | reg.addr() << 2])?;
             bus.read(&mut res)
        }).map_err(MyError::Spi)?;

        Ok(res[0])
    }

    
}

#[derive(Copy, Clone, Debug)]
pub enum MyError<SPI> {
    Spi(SPI),
    // Add other errors for your driver here.
}