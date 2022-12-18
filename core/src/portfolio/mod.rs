use crate::portfolio::dtos::Portfolio as PortfolioD;
use crate::portfolio::create_portfolio::Error as CreatePortfolioError;

pub mod entities;
pub mod create_portfolio;
pub mod dtos;


pub trait PortfolioRepository: Send + Sync {
    fn get_portfolios(&self) -> Result<Vec<PortfolioD>, ()>;
    fn add_portfolio(&self, new_portfolio: PortfolioD) -> Result<PortfolioD, CreatePortfolioError>;
    fn portfolio_exist(&self, portfolio: &PortfolioD) -> Result<bool, ()>;
}

//TODO idea: test persistence with new crate using core and persistence as deps to avoid cycle


