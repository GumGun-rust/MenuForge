use std::marker::PhantomData;

use super::RawConfigs;
use super::Fields;


#[derive(Default)]
pub struct RawSelect<Type, RetOk, RetErr> {
    pub configs: RawConfigs,
    pub fields: Fields,
    pd_0: PhantomData<Type>,
    pd_1: PhantomData<RetOk>,
    pd_2: PhantomData<RetErr>,
}

#[derive(Default)]
pub struct RawConfigs {
    pub table_size: u16,
    //exit_on_new_key:bool,
    //new_options:bool,
}

