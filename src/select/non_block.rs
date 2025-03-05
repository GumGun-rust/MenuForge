use super::KeysMut;
use super::RawSelect;


//For now this cant be updated in real time
pub struct SelectNonBlock<Type, RetOk, RetErr> {
    pub owner: Vec<Type>,
    pub keys: KeysMut<Type, RetOk, RetErr>,
    pub inner: RawSelect<Type, RetOk, RetErr>,
}

impl<Type, RetOk, RetErr> SelectNonBlock<Type, RetOk, RetErr> {
    
    
}
