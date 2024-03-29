pub enum AdresseCapteur {
    I2cAddress,
    Vmel7700DefaultI2cAddress,
}

/// Commandes pour contrôler le capteur
#[derive(Copy, Clone)]
pub enum Registre {
    AlsConfig,
    Als,
    AlsWhite,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Gain {
    AlsGain1,
    AlsGain2,
    AlsGain1_8,
    AlsGain1_4,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TempsIntegration {
    AlsIt25MS,
    AlsIt50MS,
    AlsIt100MS,
    AlsIt200MS,
    AlsIt400MS,
    AlsIt800MS,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Persistance {
    AlsPers1,
    AlsPers2,
    AlsPers4,
    AlsPers8,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ModeEconomieEnergie {
    AlsPowerSaveMode1,
    AlsPowerSaveMode2,
    AlsPowerSaveMode3,
    AlsPowerSaveMode4,
}

impl AdresseCapteur {
    pub fn adresse(&self) -> u16 {
        match self {
            AdresseCapteur::I2cAddress => 0x10,
            AdresseCapteur::Vmel7700DefaultI2cAddress => 0x10,
        }
    }
}

impl Registre {
    pub(crate) fn adresse(&self) -> u8 {
        match self {
            Registre::AlsConfig => 0x00,
            Registre::Als => 0x04,
            Registre::AlsWhite => 0x05,
        }
    }
}

impl Gain {
    pub(crate) fn adresse(&self) -> u8 {
        match self {
            Gain::AlsGain1 => 0x00,
            Gain::AlsGain2 => 0x01,
            Gain::AlsGain1_8 => 0x02,
            Gain::AlsGain1_4 => 0x03,
        }
    }

    pub(crate) fn valeur(&self) -> f64 {
        match self {
            Gain::AlsGain1 => 1.,
            Gain::AlsGain2 => 2.,
            Gain::AlsGain1_4 => 0.25,
            Gain::AlsGain1_8 => 0.125,
        }
    }

    pub(crate) fn determiner(adresse: u8) -> Gain {
        match adresse {
            0x00 => Gain::AlsGain1,
            0x01 => Gain::AlsGain2,
            0x02 => Gain::AlsGain1_8,
            0x03 => Gain::AlsGain1_4,
            _ => Gain::AlsGain1,
        }
    }

    pub(crate) fn suivant(&self) -> Self {
        match &self {
            Gain::AlsGain1 => Gain::AlsGain2,
            Gain::AlsGain2 => Gain::AlsGain2,
            Gain::AlsGain1_8 => Gain::AlsGain1_4,
            Gain::AlsGain1_4 => Gain::AlsGain1,
        }
    }
}

impl TempsIntegration {
    pub(crate) fn adresse(&self) -> u8 {
        match self {
            TempsIntegration::AlsIt25MS => 0x0C,
            TempsIntegration::AlsIt50MS => 0x08,
            TempsIntegration::AlsIt100MS => 0x00,
            TempsIntegration::AlsIt200MS => 0x01,
            TempsIntegration::AlsIt400MS => 0x02,
            TempsIntegration::AlsIt800MS => 0x03,
        }
    }

    pub(crate) fn valeur(&self) -> f64 {
        match self {
            TempsIntegration::AlsIt25MS => 25.,
            TempsIntegration::AlsIt50MS => 50.,
            TempsIntegration::AlsIt100MS => 100.,
            TempsIntegration::AlsIt200MS => 200.,
            TempsIntegration::AlsIt400MS => 400.,
            TempsIntegration::AlsIt800MS => 800.,
        }
    }

    pub(crate) fn determiner(adresse: u8) -> TempsIntegration {
        match adresse {
            0x0C => TempsIntegration::AlsIt25MS,
            0x08 => TempsIntegration::AlsIt50MS,
            0x00 => TempsIntegration::AlsIt100MS,
            0x01 => TempsIntegration::AlsIt200MS,
            0x02 => TempsIntegration::AlsIt400MS,
            0x03 => TempsIntegration::AlsIt800MS,
            _ => TempsIntegration::AlsIt100MS,
        }
    }

    pub(crate) fn precedent(&self) -> Self {
        match self {
            TempsIntegration::AlsIt25MS => TempsIntegration::AlsIt25MS,
            TempsIntegration::AlsIt50MS => TempsIntegration::AlsIt25MS,
            TempsIntegration::AlsIt100MS => TempsIntegration::AlsIt50MS,
            TempsIntegration::AlsIt200MS => TempsIntegration::AlsIt100MS,
            TempsIntegration::AlsIt400MS => TempsIntegration::AlsIt200MS,
            TempsIntegration::AlsIt800MS => TempsIntegration::AlsIt400MS,
        }
    }

    pub(crate) fn suivant(&self) -> Self {
        match self {
            TempsIntegration::AlsIt25MS => TempsIntegration::AlsIt50MS,
            TempsIntegration::AlsIt50MS => TempsIntegration::AlsIt100MS,
            TempsIntegration::AlsIt100MS => TempsIntegration::AlsIt200MS,
            TempsIntegration::AlsIt200MS => TempsIntegration::AlsIt400MS,
            TempsIntegration::AlsIt400MS => TempsIntegration::AlsIt800MS,
            TempsIntegration::AlsIt800MS => TempsIntegration::AlsIt800MS,
        }
    }
}

impl Persistance {
    pub(crate) fn adresse(&self) -> u8 {
        match self {
            Persistance::AlsPers1 => 0x00,
            Persistance::AlsPers2 => 0x01,
            Persistance::AlsPers4 => 0x02,
            Persistance::AlsPers8 => 0x03,
        }
    }

    pub(crate) fn determiner(adresse: u8) -> Persistance {
        match adresse {
            0x00 => Persistance::AlsPers1,
            0x01 => Persistance::AlsPers2,
            0x02 => Persistance::AlsPers4,
            0x03 => Persistance::AlsPers8,
            _ => Persistance::AlsPers1,
        }
    }
}

impl ModeEconomieEnergie {
    pub(crate) fn adresse(&self) -> u8 {
        match self {
            ModeEconomieEnergie::AlsPowerSaveMode1 => 0x00,
            ModeEconomieEnergie::AlsPowerSaveMode2 => 0x01,
            ModeEconomieEnergie::AlsPowerSaveMode3 => 0x02,
            ModeEconomieEnergie::AlsPowerSaveMode4 => 0x03,
        }
    }

    pub(crate) fn determiner(adresse: u8) -> ModeEconomieEnergie {
        match adresse {
            0x00 => ModeEconomieEnergie::AlsPowerSaveMode1,
            0x01 => ModeEconomieEnergie::AlsPowerSaveMode2,
            0x02 => ModeEconomieEnergie::AlsPowerSaveMode3,
            0x03 => ModeEconomieEnergie::AlsPowerSaveMode4,
            _ => ModeEconomieEnergie::AlsPowerSaveMode1,
        }
    }
}
