use std::convert::TryFrom;
use crate::portfolio::dtos::Portfolio as PortfolioDto;

pub enum ValidationError{
    Empty,
    SizeLimitReached
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone)]
pub struct PortfolioName(pub(in crate::portfolio) String);
impl From<PortfolioName> for String {
    fn from(name: PortfolioName) -> Self {
        name.0
    }
}
impl PartialEq<&PortfolioName> for String {
    fn eq(&self, other: &&PortfolioName) -> bool {
        other.0.eq(self)
    }

    fn ne(&self, other: &&PortfolioName) -> bool {
        other.0.ne(self)
    }
}

impl TryFrom<String> for PortfolioName {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError::Empty);
        }
        if value.len() > 50 {
            return Err(ValidationError::SizeLimitReached);
        }
        Ok(PortfolioName(value))
    }
}

#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub struct Portfolio{
    name: PortfolioName
}

impl Portfolio {
    pub fn new(name: PortfolioName) -> Self {
        Self {
            name
        }
    }

    pub fn get_name(&self) -> &PortfolioName {
        &self.name
    }

    pub fn into_dto(self) -> PortfolioDto {
        PortfolioDto::new(self.name.0)
    }
}