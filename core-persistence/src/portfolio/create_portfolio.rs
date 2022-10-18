

#[cfg(test)]
mod create_portfolio_test {
    use core::portfolio::create_portfolio::{Request, execute, Error};
    use core::portfolio::dtos::Portfolio;
    use persistence::portfolio::InMemoryPortfolioRepository;
    use std::sync::Arc;

    #[test]
    fn it_should_return_portfolio_name() {
        let repo = Arc::new(InMemoryPortfolioRepository::new());
        let name = "Portfolio1".to_string();
        let req = Request {
            name: name.clone()
        };
        let res = execute(req, repo.clone());
        assert_eq!(Ok(Portfolio::new(name)), res);
    }

    #[test]
    fn it_should_return_error_name_empty() {
        let repo = Arc::new(InMemoryPortfolioRepository::new());
        let name = "".to_string();
        let req = Request {
            name: name.clone()
        };
        let res = execute(req, repo.clone());
        assert_eq!(Err(Error::Empty), res);
    }

    #[test]
    fn it_should_return_error_name_oversized() {
        let repo = Arc::new(InMemoryPortfolioRepository::new());
        let name = "012345678901234567890123456789012345678901234567890".to_string();
        let req = Request {
            name: name.clone()
        };
        let res = execute(req, repo.clone());
        assert_eq!(Err(Error::SizeLimitReached), res);
    }

    #[test]
    fn it_should_return_error_already_exist() {
        let repo = Arc::new(InMemoryPortfolioRepository::new());
        let name = "Portfolio1".to_string();
        let req = Request {
            name: name.clone()
        };
        execute(req.clone(), repo.clone());
        let res = execute(req.clone(), repo.clone());
        assert_eq!(Err(Error::AlreadyExist), res);
    }

}