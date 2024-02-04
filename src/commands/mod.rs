pub enum Command {
    RESET = 0b0000_0110,     //Resets the config registers to all zero
    START = 0b0000_1000,     //Starts a single conversion or the continous conversions
    POWERDOWN = 0b0000_0010, //Power down
    RDATA = 0b0001_0000,     //Loads the output shift register with the most recent result
    RREG = 0b0010_0000,      //0010_rrnn / rr -> address / nn -> number of bytes - 1
    WREG = 0b0100_0000,      //0100_rrnn / rr -> address / nn -> number of bytes - 1
}

impl Command {
    pub fn bits(self) -> u8 {
        self as u8
    }
}
