use crate::portfolio::entities::{Portfolio, PortfolioName};

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

fn validate_portfolio(
    req: &Request,
    // name_already_exist_checker: CheckPortfolioAlreadyExist
) -> Result<(), Error>
{
    if req.name.is_empty() {
        return Err(Error::Empty);
    }
    if req.name.len() > 50 {
        return Err(Error::SizeLimitReached);
    }
    // if name_already_exist_checker(&name) {
    //     return Err(Error::AlreadyExist);
    // }
    Ok(())
}

pub fn execute(
    request: Request,
    // name_already_exist_checker: CheckPortfolioAlreadyExist
) -> Result<PortfolioName, Error>
{
    let validated = validate_portfolio(&request);
    validated.map(|_| PortfolioName(request.name))
    // validated.map(|_| Portfolio(name))
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