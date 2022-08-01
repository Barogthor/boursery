use crate::portfolio::entities::{Portfolio, PortfolioName, ValidationError};
use std::convert::TryFrom;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Error {
    Empty,
    SizeLimitReached,
    AlreadyExist
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Request {
    name: String
}

pub type CheckPortfolioAlreadyExist = fn(&String) -> bool;

pub fn execute(
    request: Request,
    // name_already_exist_checker: CheckPortfolioAlreadyExist
) -> Result<PortfolioName, Error>
{
    let validated = PortfolioName::try_from(request.name);
    validated.map_err(|err| match err {
        ValidationError::Empty => Error::Empty,
        ValidationError::SizeLimitReached => Error::SizeLimitReached
    })
}


#[cfg(test)]
mod create_portfolio_test {
    use crate::portfolio::create_portfolio::{Request, execute, Error};
    use crate::portfolio::entities::PortfolioName;

    #[test]
    fn it_should_return_portfolio_name() {
        let name = "Portfolio1".to_string();
        let req = Request {
            name: name.clone()
        };
        let res = execute(req);
        assert_eq!(Ok(PortfolioName(name)), res);
    }

    #[test]
    fn it_should_return_error_name_empty() {
        let name = "".to_string();
        let req = Request {
            name: name.clone()
        };
        let res = execute(req);
        assert_eq!(Err(Error::Empty), res);
    }

    #[test]
    fn it_should_return_error_name_oversized() {
        let name = "012345678901234567890123456789012345678901234567890".to_string();
        let req = Request {
            name: name.clone()
        };
        let res = execute(req);
        assert_eq!(Err(Error::SizeLimitReached), res);
    }

}