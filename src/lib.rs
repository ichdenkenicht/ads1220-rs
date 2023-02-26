#[deny(missing_docs)]
extern crate embedded_hal as hal;

use hal::spi::blocking::{SpiBus, SpiBusRead, SpiBusWrite, SpiDevice};

mod commands;
use commands::Command;

mod registers;
use registers::{*, BitFlags as BF};

#[derive(Debug, Copy, Clone, Default)]
pub struct Config {
    pub bits: u8,
}

impl Config {
    pub fn with_high(&self, mask: u8) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    pub fn with_low(&self, mask: u8) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}

pub struct ADS1220<SPI> {
    spi: SPI,
    config0: Config,
    config1: Config,
    config2: Config,
    config3: Config,
}

impl<SPI> ADS1220<SPI>
where
    SPI: SpiDevice,
    SPI::Bus: SpiBus, // or SpiBusRead/SpiBusWrite if you only need to read or only write.
{
    pub fn new(spi: SPI) -> Self {
        Self {
            spi,
            config0: Config::default(),
            config1: Config::default(),
            config2: Config::default(),
            config3: Config::default(),
        }
    }

    pub fn reset(&mut self) -> Result<(), MyError<SPI::Error>> {
        self.spi
            .transaction(|bus| bus.write(&[Command::RESET.bits()]))
            .map_err(MyError::Spi)?;
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), MyError<SPI::Error>> {
        self.spi
            .transaction(|bus| bus.write(&[Command::START.bits()]))
            .map_err(MyError::Spi)?;
        Ok(())
    }

    pub fn powerdown(&mut self) -> Result<(), MyError<SPI::Error>> {
        self.spi
            .transaction(|bus| bus.write(&[Command::POWERDOWN.bits()]))
            .map_err(MyError::Spi)?;
        Ok(())
    }

    fn read_register(&mut self, reg: Register) -> Result<u8, MyError<SPI::Error>> {
        let mut res: [u8; 1] = [0];

        // `transaction` asserts and deasserts CS for us. No need to do it manually!
        self.spi
            .transaction(|bus| {
                bus.write(&[Command::RREG.bits() | reg.addr() << 2])?;
                bus.read(&mut res)
            })
            .map_err(MyError::Spi)?;

        Ok(res[0])
    }

    fn write_register(&mut self, reg: Register, value: u8) -> Result<(), MyError<SPI::Error>> {
        // `transaction` asserts and deasserts CS for us. No need to do it manually!
        self.spi
            .transaction(|bus| {
                bus.write(&[Command::WREG.bits() | reg.addr() << 2])?;
                bus.write(&[value])
            })
            .map_err(MyError::Spi)?;

        Ok(())
    }

    fn read(&mut self) -> Result<u32, MyError<SPI::Error>> {
        let mut res = [0u8; 4];
        self.spi
            .transaction(|bus| {
                bus.write(&[Command::RDATA.bits()])?;
                bus.read(&mut res[1..])
            })
            .map_err(MyError::Spi)?;
        Ok(u32::from_be_bytes(res))
    }

    ///Set Programmable Gain Bypass
    pub fn set_pga_bypass(&mut self, bp: PGA_BYPASS) -> Result<(), MyError<SPI::Error>> {
        let config = match bp {
            PGA_BYPASS::ENABLED => {
                self.config0.with_low(BF::PB)
            },
            PGA_BYPASS::DISABLED => {
                self.config0.with_high(BF::PB)
            },
        };
        self.write_register(Register::CONFIG0, config.bits)?;
        self.config0 = config;
        Ok(())
    }
    
    ///Set DataReady
    pub fn set_drdym(&mut self, drdym: DRDYM) -> Result<(), MyError<SPI::Error>> {
        let config = match drdym {
            DRDYM::DRDY => {
                self.config3.with_low(BF::DRDYM)
            },
            DRDYM::DOUT_DRDY => {
                self.config3.with_high(BF::DRDYM)
            },
        };
        self.write_register(Register::CONFIG3, config.bits)?;
        self.config3 = config;
        Ok(())
    }
    
    ///Set Low-side Power Switch
    pub fn set_psw(&mut self, psw: PSW) -> Result<(), MyError<SPI::Error>> {
        let config = match psw {
            PSW::OPEN => {
                self.config2.with_low(BF::PSW)
            },
            PSW::CLOSING => {
                self.config2.with_high(BF::PSW)
            },
        };
        self.write_register(Register::CONFIG2, config.bits)?;
        self.config2 = config;
        Ok(())
    }
    
    ///Set Conversion Mode
    pub fn set_mode(&mut self, mode: CM) -> Result<(), MyError<SPI::Error>> {
        let config = match mode {
            CM::SINGLE => {
                self.config1.with_low(BF::CM)
            },
            CM::CONTINUOUS => {
                self.config1.with_high(BF::CM)
            },
        };
        self.write_register(Register::CONFIG1, config.bits)?;
        self.config1 = config;
        Ok(())
    }
    
    ///Set Temperature sensor mode
    pub fn set_temp_mode(&mut self, mode: TS) -> Result<(), MyError<SPI::Error>> {
        let config = match mode {
            TS::DISABLED => {
                self.config1.with_low(BF::TS)
            },
            TS::ENABLED => {
                self.config1.with_high(BF::TS)
            },
        };
        self.write_register(Register::CONFIG1, config.bits)?;
        self.config1 = config;
        Ok(())
    }
    
    pub fn set_gain(&mut self, gain: PGA) -> Result<(), MyError<SPI::Error>> {
        
        let config = match gain {
            PGA::Gain1 => {
                self.config0.with_low(0x03 << 1)
            },
            PGA::Gain2 => {
                self.config0.with_low(0x03 << 1);
                self.config0.with_high(PGA::Gain2.bits_on_pos())
            },
            PGA::Gain4 => {
                self.config0.with_low(0x03 << 1);
                self.config0.with_high(PGA::Gain4.bits_on_pos())
            },
            PGA::Gain8 => {
                self.config0.with_low(0x03 << 1);
                self.config0.with_high(PGA::Gain8.bits_on_pos())
            },
            PGA::Gain16 => {
                self.config0.with_low(0x03 << 1);
                self.config0.with_high(PGA::Gain16.bits_on_pos())
            },
            PGA::Gain32 => {
                self.config0.with_low(0x03 << 1);
                self.config0.with_high(PGA::Gain32.bits_on_pos())
            },
            PGA::Gain64 => {
                self.config0.with_low(0x03 << 1);
                self.config0.with_high(PGA::Gain64.bits_on_pos())
            },
            PGA::Gain128 => {
                //self.config0.with_low(0x03 << 1);
                self.config0.with_high(PGA::Gain128.bits_on_pos())
            },
        };
        self.write_register(Register::CONFIG0, config.bits)?;
        self.config0 = config;
        
        Ok(())
    }
    
    pub fn set_channel(&mut self, ch: Channel) -> Result<(), MyError<SPI::Error>> {
        
        let config = match ch {
            Channel::DIFF_AIN0_AIN1 => {
                self.config0.with_low(0xF0);
            },
            Channel::DIFF_AIN0_AIN2 => {
                self.config0.with_low(0xF0);
                self.config0.with_high(Channel::DIFF_AIN0_AIN2.bits_on_pos());
            },
            Channel::DIFF_AIN0_AIN3 => {
                self.config0.with_low(0xF0);
                self.config0.with_high(Channel::DIFF_AIN0_AIN3.bits_on_pos());
            },
            Channel::DIFF_AIN1_AIN2 => {
                self.config0.with_low(0xF0);
                self.config0.with_high(Channel::DIFF_AIN1_AIN2.bits_on_pos());
            },
            Channel::DIFF_AIN1_AIN3 => {
                self.config0.with_low(0xF0);
                self.config0.with_high(Channel::DIFF_AIN1_AIN3.bits_on_pos());
            },
            Channel::DIFF_AIN2_AIN3 => {
                self.config0.with_low(0xF0);
                self.config0.with_high(Channel::DIFF_AIN2_AIN3.bits_on_pos());
            },
            Channel::DIFF_AIN1_AIN0 => {
                self.config0.with_low(0xF0);
                self.config0.with_high(Channel::DIFF_AIN1_AIN0.bits_on_pos());
            },
            Channel::DIFF_AIN3_AIN2 => {
                self.config0.with_low(0xF0);
                self.config0.with_high(Channel::DIFF_AIN3_AIN2.bits_on_pos());
            },
            Channel::AIN0 => {
                self.config0.with_low(0xF0);
                self.config0.with_high(Channel::AIN0.bits_on_pos());
            },
            Channel::AIN1 => {
                self.config0.with_low(0xF0);
                self.config0.with_high(Channel::AIN1.bits_on_pos());
            },
            Channel::AIN2 => {
                self.config0.with_low(0xF0);
                self.config0.with_high(Channel::AIN2.bits_on_pos());
            },
            Channel::AIN3 => {
                self.config0.with_low(0xF0);
                self.config0.with_high(Channel::AIN3.bits_on_pos());
            },
            Channel::VREFDIFF4 => {
                self.config0.with_low(0xF0);
            },
            Channel::AVDIFF4 => {
                self.config0.with_low(0xF0);
            },
            Channel::VCC2 => {
                self.config0.with_low(0xF0);
            },
            Channel::RESERVED => {
                self.config0.with_low(0xF0);
            },
            
        };
        
        
        self.write_register(Register::CONFIG0, config.bits)?;
        self.config0 = config;
        
        Ok(())
    }
    
}

#[derive(Copy, Clone, Debug)]
pub enum MyError<SPI> {
    Spi(SPI),
    // Add other errors for your driver here.
}
