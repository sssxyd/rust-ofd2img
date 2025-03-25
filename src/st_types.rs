use std::{num::ParseFloatError, str::FromStr};

#[derive(Debug)]
pub enum ParseSTError {
    InvalidFormat,
    ParseFloatError(ParseFloatError),
}

#[derive(Debug, Default, Clone)]
pub struct STPos {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl FromStr for STPos {
    type Err = ParseSTError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts : Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(ParseSTError::InvalidFormat);
        }
        // 解析 x 和 y
        let x = parts[0]
            .parse::<f64>()
            .map_err(ParseSTError::ParseFloatError)?;
        let y = parts[1]
            .parse::<f64>()
            .map_err(ParseSTError::ParseFloatError)?;

        Ok(STPos { x, y })
    }
}

#[derive(Debug, Default, Clone)]
pub struct STBox {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) w: f64,
    pub(crate) h: f64,
}

impl FromStr for STBox {
    type Err = ParseSTError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts : Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 4 {
            return Err(ParseSTError::InvalidFormat);
        }
        // 解析 x, y, w, h
        let x = parts[0]
            .parse::<f64>()
            .map_err(ParseSTError::ParseFloatError)?;
        let y = parts[1]
            .parse::<f64>()
            .map_err(ParseSTError::ParseFloatError)?;
        let w = parts[2]
            .parse::<f64>()
            .map_err(ParseSTError::ParseFloatError)?;
        let h = parts[3]
            .parse::<f64>()
            .map_err(ParseSTError::ParseFloatError)?;

        Ok(STBox { x, y, w, h })
    }
}
