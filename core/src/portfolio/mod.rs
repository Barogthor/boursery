use crate::portfolio::entities::{Portfolio, PortfolioName};
use crate::portfolio::create_portfolio::Error as CreatePortfolioError;

pub mod entities;
pub mod create_portfolio;
pub mod dtos;


pub trait PortfolioRepository: Send + Sync {
    fn get_portfolios(&self) -> Result<Vec<Portfolio>, ()>;
    fn add_portfolio(&self, name: PortfolioName) -> Result<PortfolioName, CreatePortfolioError>;
    fn portfolio_exist(&self, name: &PortfolioName) -> Result<bool, ()>;
}

//TODO idea: test persistence with new crate using core and persistence as deps to avoid cycle


