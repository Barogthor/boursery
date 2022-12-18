use crate::portfolio::entities::{PortfolioName, ValidationError};
use crate::portfolio::dtos::Portfolio as PortfolioDTO;
use std::convert::TryFrom;
use crate::portfolio::PortfolioRepository;
use std::sync::Arc;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Error {
    Empty,
    SizeLimitReached,
    AlreadyExist,
    Other(String)
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Request {
    pub name: String
}

pub type CheckPortfolioAlreadyExist = fn(&String) -> bool;

pub fn execute(
    request: Request,
    repository: Arc<dyn PortfolioRepository>
) -> Result<PortfolioDTO, Error>
{
    let validated = PortfolioName::try_from(request.name)
        .map_err(|err| match err {
            ValidationError::Empty => Error::Empty,
            ValidationError::SizeLimitReached => Error::SizeLimitReached
        })?;
    let dto = PortfolioDTO::new(validated.0);
        // .map(Portfolio::new).map(Portfolio::into_dto)
    repository.add_portfolio(dto)
}