#[derive(Debug, PartialOrd, PartialEq)]
pub struct PortfolioName(pub(in crate::portfolio) String);

#[derive(PartialOrd, PartialEq, Debug)]
pub struct Portfolio{
    name: PortfolioName
}