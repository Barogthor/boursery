use std::convert::TryFrom;

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
}