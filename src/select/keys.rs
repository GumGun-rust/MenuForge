use super::SelOk;
use super::Select;
use super::KeyFuncMut;
use super::KeyFunc;
use super::SelectActCtx;
use super::SelectKeysDS;

use std::marker::PhantomData;
use std::collections::HashMap;

use crossterm::event;

pub trait KeysTrait<Type, ActCtx, RetOk, RetErr> {
    fn get_key_action(&mut self, _:&mut ActCtx, _:&event::Event) -> Option<KeyFunc<Type, ActCtx, RetOk, RetErr>>;
}

pub trait KeysTraitMut<Type, ActCtx, RetOk, RetErr> {
    fn get_key_action_mut(&mut self, _:&mut ActCtx, _:&event::Event) -> Option<KeyFuncMut<Type, ActCtx, RetOk, RetErr>>;
}

//use derivative::Derivative;
/*
#[derive(Derivative)]
#[derivative(Default)]
pub struct KeysMut<Type, RetOk, RetErr> {
    #[derivative(Default(bound=""))]
    keys: HashMap<event::Event, KeyFuncMut<Type, RetOk, RetErr>>,
}

impl<Type, RetOk, RetErr> KeysMut<Type, RetOk, RetErr> {
    pub(super) fn keys_get(&self) -> &HashMap<event::Event, KeyFuncMut<Type, RetOk, RetErr>>{
        &self.keys
    }
}

impl<Type> KeysMut<Type, SelOk, ()> {
    pub fn default_keys() -> Self {
        let mut keys = Self::default();
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Char('k'),
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.keys.insert(key, Self::move_cursor_up), None);
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Char('j'),
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.keys.insert(key, Self::move_cursor_down), None);
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Enter,
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.keys.insert(key, Self::exit), None);
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.keys.insert(key, Self::abort), None);
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.keys.insert(key, Self::abort), None);
        keys
    }
    #[allow(dead_code)]
    fn exit(_:&mut Type, _:usize, _:&mut usize) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Exit)
    }
    
    #[allow(dead_code)]
    fn nope(_:&mut Type, _:usize, _:&mut usize) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn move_cursor_down(_:&mut Type, size:usize, index:&mut usize) -> Result<SelOk, SelErr<()>> {
        if *index < size-1 {
            *index += 1;
        }
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn move_cursor_up(_:&mut Type, _:usize, index:&mut usize) -> Result<SelOk, SelErr<()>> {
        if *index > 0 {
            *index -= 1;
        }
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn abort(_:&mut Type, _:usize, _:&mut usize) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Abort)
    }
}
*/

type KeyCbk<Type, KeyType, ActCtx, RetOk, RetErr> = fn(&mut KeyType, &mut ActCtx, &event::Event) -> Option<KeyFunc<Type, ActCtx, RetOk, RetErr>>;

pub struct Keys<Type, KeyType, ActCtx, RetOk, RetErr> {
    data_holder: KeyType, 
    function: KeyCbk<Type, KeyType, ActCtx, RetOk, RetErr>,
    pd_0: PhantomData<Type>,
    pd_1: PhantomData<KeyType>,
    pd_2: PhantomData<ActCtx>,
    pd_3: PhantomData<RetOk>,
    pd_4: PhantomData<RetErr>,
}

impl<Type, KeyType, ActCtx, RetOk, RetErr> KeysTrait<Type, ActCtx, RetOk, RetErr> for Keys<Type, KeyType, ActCtx, RetOk, RetErr> {
    fn get_key_action(&mut self, ctx:&mut ActCtx, event:&event::Event) -> Option<KeyFunc<Type, ActCtx, RetOk, RetErr>> {
        (self.function)(&mut self.data_holder, ctx, event)
    }
}
    
impl<Type, KeyType, ActCtx, RetOk, RetErr> Keys<Type, KeyType, ActCtx, RetOk, RetErr> {
    
    pub(super) fn new(data_holder:KeyType, fetch_cbk:KeyCbk<Type, KeyType, ActCtx, RetOk, RetErr>) -> Self {
        Self{
            data_holder: data_holder,
            function: fetch_cbk,
            pd_0: PhantomData,
            pd_1: PhantomData,
            pd_2: PhantomData,
            pd_3: PhantomData,
            pd_4: PhantomData,
        }
    }
}

impl<'x, Type:std::fmt::Display> Keys<Type, SelectKeysDS<'x, Type>, SelectActCtx<'x>, SelOk, ()> {
    
    fn function_cbk<'a, 'c>(data_holder:&'a mut SelectKeysDS<'c, Type>, _action_ctx:&'a mut SelectActCtx<'c>, event:&'a event::Event) -> Option<KeyFunc<Type, SelectActCtx<'c>, SelOk, ()>> {
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
    
}
