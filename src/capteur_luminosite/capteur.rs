use rppal::i2c::I2c;

use crate::capteur_luminosite::instruction::{
    AdresseCapteur, GainValues, Instruction, IntegrationTimeValues,
};

pub struct Veml7700 {
    i2c: I2c,
    big_endian: bool,
}

impl Veml7700 {
    pub fn new() -> Result<Self, rppal::i2c::Error> {
        let big_endian;
        if cfg!(target_endian = "big") {
            big_endian = true;
        } else {
            big_endian = false;
        }

        Ok(Self {
            i2c: I2c::new()?,
            big_endian,
        })
    }

    pub fn demarrer(&mut self) -> Result<(), rppal::i2c::Error> {
        self.i2c
            .set_slave_address(AdresseCapteur::I2cAddress.adresse())?;

        let gain = Instruction::AlsGain1_8.adresse();
        let integration_time = Instruction::Als100MS.adresse();
        let persistance = 0x00;
        let interrupt_enable = 0x00;
        let shutdown = 0x00;

        let config_data = gain << 11
            | integration_time << 6
            | persistance << 4
            | interrupt_enable << 1
            | shutdown << 0;

        let config_data = config_data.to_le_bytes();
        println!("configuration {:?}",config_data);

        self.i2c
            .block_write(Instruction::AlsConfig as u8, &config_data)
            .unwrap();

        let mut buffer = [0u8; 2];
        self.i2c.block_read(Instruction::Als as u8, &mut buffer)?;
        print!("buffer {:?}", buffer);

        let mut buffer = [0u8; 2];
        self.i2c.block_read(Instruction::Als as u8, &mut buffer)?;
        print!("buffer {:?}", buffer);
        Ok(())
    }
}