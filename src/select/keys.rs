use super::SelOk;
use super::SelErr;
//use super::KeyFuncMut;
use super::KeyFunc;
use super::SelectCtx;

use std::collections::HashMap;

use crossterm::event;

use derivative::Derivative;

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

#[derive(Derivative)]
#[derivative(Default)]
pub struct Keys<Type, ActCtx, RetOk, RetErr> {
    #[derivative(Default(bound=""))]
    keys: HashMap<event::Event, KeyFunc<Type, ActCtx, RetOk, RetErr>>,
}

impl<Type, ActCtx, RetOk, RetErr> Keys<Type, ActCtx, RetOk, RetErr> {
    pub(super) fn keys_get(&self) -> &HashMap<event::Event, KeyFunc<Type, ActCtx, RetOk, RetErr>>{
        &self.keys
    }
}

impl<Type> Keys<Type, SelectCtx<'_>, SelOk, ()> {
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
    fn exit(_:&[Type], _:&mut SelectCtx) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Exit)
    }
    
    #[allow(dead_code)]
    fn nope(_:&[Type], _:&mut SelectCtx) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn move_cursor_down(_:&[Type], modi:&mut SelectCtx) -> Result<SelOk, SelErr<()>> {
        let (size, index) = modi;
        if **index < *size-1 {
            **index += 1;
        }
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn move_cursor_up(_:&[Type], modi:&mut SelectCtx) -> Result<SelOk, SelErr<()>> {
        let (_, index) = modi;
        if **index > 0 {
            **index -= 1;
        }
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn abort(_:&[Type], _:&mut SelectCtx) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Abort)
    }
}

