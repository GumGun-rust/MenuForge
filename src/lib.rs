mod select;
mod compat;
mod keys;

pub use select::Select;
pub use select::SelConf;

//pub use select::SelectNonBlock;
pub use select::RawSelect;
pub use select::RawConfigs;
pub use select::RawSelResult;
pub use select::KeysTrait;
pub use select::KeyFunc;
pub use select::SelErr;
pub use crossterm;


/*
fn add(left: u64, right: u64) -> u64 {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
