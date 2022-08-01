// use core::portfolio::{PortfolioEntity, PortfolioRepository};
use std::collections::HashMap;
use core::portfolio::entities::{Portfolio, PortfolioName};
use std::sync::{Mutex, PoisonError, MutexGuard};
use core::portfolio::PortfolioRepository;

enum DbError {
    Duplicate,
    MultipleResultNotExpected,
    LockError
}

pub struct InMemoryPortfolioRepository {
    repo: Mutex<Vec<Portfolio>>,
}

impl InMemoryPortfolioRepository {
    pub fn new() -> Self {
        Self {
            repo: Default::default(),
        }
    }
}

impl PortfolioRepository for InMemoryPortfolioRepository {
    fn get_portfolios(&self) -> Result<Vec<Portfolio>, ()> {
        let lock = match self.repo.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(())
        };
        let mut vec = lock.to_vec();
        Ok(vec)
    }

    fn add_portfolio(&self, name: PortfolioName) -> Result<PortfolioName, ()> {
        let mut lock = match self.repo.lock() {
            Ok(lock) => lock,
            _ => return Err(())
        };
        if self.portfolio_exist(&name).unwrap() {
            return Err(());
        }
        lock.push(Portfolio::new(name.clone()));
        Ok(name)
    }

    fn portfolio_exist(&self, name: &PortfolioName) -> Result<bool, ()> {
        let lock = match self.repo.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(())
        };
        Ok(lock.iter().any(|portfolio| portfolio.get_name() == name))
    }
}
//
// impl PortfolioRepository for InMemoryPortfolioRepository {
//     fn get_portfolios(&self) -> Result<Vec<PortfolioEntity>, ()> {
//         let v: Vec<_> = self.repo.values().map(to_domain).collect();
//         Ok(v)
//     }
//
//     fn add_portfolio(&mut self, portfolio: PortfolioEntity) -> Result<(), ()> {
//         let mut record = from_domain(&portfolio);
//         record.id = self.next_id;
//         self.repo.insert(self.next_id, record);
//         self.next_id += 1;
//         Ok(())
//     }
//
//     fn portfolio_exist(&self, portfolio: &PortfolioEntity) -> Result<bool, ()> {
//         let entity_db = from_domain(portfolio);
//         let exist = self.repo.values().find(|it| it.eq(&&entity_db)).is_some();
//         if exist {
//             Err(())
//         } else {
//             Ok(exist)
//         }
//     }
// }
//
// fn from_domain(portfolio: &PortfolioEntity) -> PortfolioEntityRecord {
//     PortfolioEntityRecord {
//         id: portfolio.id,
//         name: portfolio.name.clone(),
//     }
// }
//
// fn to_domain(record: &PortfolioEntityRecord) -> PortfolioEntity {
//     PortfolioEntity {
//         id: record.id,
//         name: record.name.clone()
//     }
// }
