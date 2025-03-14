use super::SelOk;
use super::SelErr;
use super::Fields;
//use super::KeyFuncMut;
use super::KeyFunc;
use super::SelectContext;

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
pub struct Keys<Type, Context, RetOk, RetErr> {
    #[derivative(Default(bound=""))]
    keys: HashMap<event::Event, KeyFunc<Type, Context, RetOk, RetErr>>,
}

impl<Type, Context, RetOk, RetErr> Keys<Type, Context, RetOk, RetErr> {
    pub(super) fn keys_get(&self) -> &HashMap<event::Event, KeyFunc<Type, Context, RetOk, RetErr>>{
        &self.keys
    }
}

impl<'a, Type> Keys<Type, SelectContext<'a>, SelOk, ()> {
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
    fn exit(_:&[Type], _:usize, _:&mut usize, _:SelectContext<'a>) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Exit)
    }
    
    #[allow(dead_code)]
    fn nope(_:&[Type], _:usize, _:&mut usize, _:SelectContext<'a>) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn move_cursor_down(_:&[Type], size:usize, index:&mut usize, modi:SelectContext<'a>) -> Result<SelOk, SelErr<()>> {
        let (size, index) = modi;
        if *index < size-1 {
            *index += 1;
        }
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn move_cursor_up(_:&[Type], _:usize, index:&mut usize, modi:SelectContext<'a>) -> Result<SelOk, SelErr<()>> {
        let (_, index) = modi;
        if *index > 0 {
            *index -= 1;
        }
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn abort(_:&[Type], _:usize, _:&mut usize, _:SelectContext<'a>) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Abort)
    }
    
}

