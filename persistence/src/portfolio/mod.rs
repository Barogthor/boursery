// use core::portfolio::{PortfolioEntity, PortfolioRepository};
use std::collections::HashMap;
use core::portfolio::entities::{Portfolio as PortfolioE, PortfolioName};
use std::sync::{Mutex, PoisonError, MutexGuard};
use core::portfolio::PortfolioRepository;
use core::portfolio::dtos::Portfolio as PortfolioD;
use core::portfolio::create_portfolio::Error as CreatePortfolioError;

pub enum DbError {
    Duplicate,
    MultipleResultNotExpected,
    LockError
}

pub struct InMemoryPortfolioRepository {
    repo: Mutex<Vec<PortfolioD>>,
}

impl InMemoryPortfolioRepository {
    pub fn new() -> Self {
        Self {
            repo: Default::default(),
        }
    }

    fn portoflio_exist_priv(&self, lock: &MutexGuard<Vec<PortfolioD>>, name: &PortfolioName) -> Result<bool, ()>{
        Ok(lock.iter().any(|portfolio| portfolio.name == name))
    }
}

impl PortfolioRepository for InMemoryPortfolioRepository {
    fn get_portfolios(&self) -> Result<Vec<PortfolioE>, ()> {
        let lock = match self.repo.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(())
        };
        // let mut vec = lock.to_vec().into_iter().map(PortfolioD::into_domain).collect();
        Ok(vec![])
    }

    fn add_portfolio(&self, name: PortfolioName) -> Result<PortfolioName, CreatePortfolioError> {
        let mut lock = match self.repo.lock() {
            Ok(lock) => lock,
            _ => return Err(CreatePortfolioError::Other(format!("lock error")))
        };
        if self.portoflio_exist_priv(&lock, &name).unwrap() {
            return Err(CreatePortfolioError::AlreadyExist);
        }
        // lock.push(PortfolioD::new(name));
        Ok(name)
    }

    fn portfolio_exist(&self, name: &PortfolioName) -> Result<bool, ()> {
        let lock = match self.repo.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(())
        };
        self.portoflio_exist_priv(&lock, name)
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
