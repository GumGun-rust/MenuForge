use super::KeyFunc;

use super::SelectOk;
use super::Select;
use super::SelectActCtx;
use super::SelectKeysDS;

use std::fmt::Display;
use std::marker::PhantomData;
use std::collections::HashMap;

use crossterm::event;

pub trait KeysTrait<Type, ActCtx, RetOk, RetErr> {
    fn get_key_action(&mut self, _:&mut ActCtx, _:&event::Event) -> Option<KeyFunc<Type, ActCtx, RetOk, RetErr>>;
}

type KeyCbk<Type, KeyType, ActCtx, RetOk, RetErr> = fn(&mut KeyType, &mut ActCtx, &event::Event) -> Option<KeyFunc<Type, ActCtx, RetOk, RetErr>>;

pub struct Keys<Type, KeyType, ActCtx, RetOk, RetErr> {
    data_holder: KeyType, 
    function: KeyCbk<Type, KeyType, ActCtx, RetOk, RetErr>,
    default: Option<KeyFunc<Type, ActCtx, RetOk, RetErr>>,
    pd_0: PhantomData<Type>,
    pd_1: PhantomData<KeyType>,
    pd_2: PhantomData<ActCtx>,
    pd_3: PhantomData<RetOk>,
    pd_4: PhantomData<RetErr>,
}

impl<Type, KeyType, ActCtx, RetOk, RetErr> KeysTrait<Type, ActCtx, RetOk, RetErr> for Keys<Type, KeyType, ActCtx, RetOk, RetErr> {
    fn get_key_action(&mut self, ctx:&mut ActCtx, event:&event::Event) -> Option<KeyFunc<Type, ActCtx, RetOk, RetErr>> {
        let holder = (self.function)(&mut self.data_holder, ctx, event);
        if holder.is_some() {
            return holder;
        }
        self.default
    }
}
    
impl<Type, KeyType, ActCtx, RetOk, RetErr> Keys<Type, KeyType, ActCtx, RetOk, RetErr> {
    
    pub(super) fn new(data_holder:KeyType, fetch_cbk:KeyCbk<Type, KeyType, ActCtx, RetOk, RetErr>) -> Self {
        Self{
            data_holder: data_holder,
            function: fetch_cbk,
            default: None,
            pd_0: PhantomData,
            pd_1: PhantomData,
            pd_2: PhantomData,
            pd_3: PhantomData,
            pd_4: PhantomData,
        }
    }
}


impl<'x, Type:Display> Keys<Type, SelectKeysDS<'x, Type>, SelectActCtx<'x>, SelectOk, ()> {
    
    fn function_cbk<'a, 'c>(data_holder:&'a mut SelectKeysDS<'c, Type>, _action_ctx:&'a mut SelectActCtx<'c>, event:&'a event::Event) -> Option<KeyFunc<Type, SelectActCtx<'c>, SelectOk, ()>> {
        data_holder.get(event).copied()
    }
    
    pub fn default_keys() -> Self {
        let mut keys = Self::new(HashMap::default(), Self::function_cbk);
        
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Char('k'),
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.data_holder.insert(key, Select::move_cursor_up), None);
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Char('j'),
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.data_holder.insert(key, Select::move_cursor_down), None);
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Enter,
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.data_holder.insert(key, Select::exit), None);
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.data_holder.insert(key, Select::abort), None);
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.data_holder.insert(key, Select::abort), None);
        keys
    }
    
    pub fn set_default(&mut self) -> bool {
        let holder = match self.default {
            Some(_) => true,
            None => false
        };
        self.default = Some(Select::abort);
        holder
    }

    pub fn ignore_extra_keys(&mut self) -> bool {
        let holder = match self.default {
            Some(_) => true,
            None => false
        };
        self.default = Some(Select::nope);
        holder
    }
    
}
