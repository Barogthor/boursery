use crate::portfolio::entities::{Portfolio as PortfolioD, PortfolioName};
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Portfolio {
    pub name: String
}

impl Portfolio {
    pub fn new (name :String) -> Self {
        Self {
            name
        }
    }

    pub fn into_domain(self) -> Result<PortfolioD, ()> {
        let Portfolio {name, ..} = self;
        let name = PortfolioName::try_from(name);
        match name {
            Ok(name) => Ok(PortfolioD::new(name)),
            _ => Err(())
        }

    }
}