mod select;
mod compat;
mod keys;

/* re exports */
pub use crossterm;

pub use select::Select;
pub use select::SelErr;
pub use select::KeysTrait;
pub use select::KeyFunc;


#[cfg(feature = "raw")]
pub use raw_export::*;

#[cfg(feature = "raw")]
mod raw_export {
    use super::select;

    pub use select::RawSelect;
    pub use select::RawConfigs;
    pub use select::RawSelResult;
}

