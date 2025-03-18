use super::SelOk;
use super::SelErr;
//use super::KeyFuncMut;
use super::KeyFunc;
use super::SelectActCtx;
use super::SelectKeysDS;

use std::marker::PhantomData;
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

//type KeyCbk<Type, KeyType, ActCtx, RetOk, RetErr> = for<'a> fn(&'a mut KeyType, &'a mut ActCtx, event:&'a event::Event) -> Option<KeyFunc<Type, ActCtx, RetOk, RetErr>>;
type KeyCbk<Type, KeyType, ActCtx, RetOk, RetErr> = fn(&mut KeyType, &mut ActCtx, &event::Event) -> Option<KeyFunc<Type, ActCtx, RetOk, RetErr>>;

//#[derive(Derivative)]
//#[derivative(Default)]
pub struct Keys<Type, KeyType, ActCtx, RetOk, RetErr> {
    //#[derivative(Default(bound=""))]
    //TODO: Change this field for a generic

    data_holder: KeyType, //HashMap<event::Event, KeyFunc<Type, ActCtx, RetOk, RetErr>>,
    //function: KeyCbk<Type, KeyType, ActCtx, RetOk, RetErr>,
    function: KeyCbk<Type, KeyType, ActCtx, RetOk, RetErr>,
    pd_0: PhantomData<Type>,
    pd_1: PhantomData<KeyType>,
    pd_2: PhantomData<ActCtx>,
    pd_3: PhantomData<RetOk>,
    pd_4: PhantomData<RetErr>,
}

impl<Type, KeyType, ActCtx, RetOk, RetErr> Keys<Type, KeyType, ActCtx, RetOk, RetErr> {
    
    //pub(super) fn new(b: KeyCbk<Type, KeyType, ActCtx, RetOk, RetErr>) -> Self {
    pub(super) fn new(data_holder:KeyType, b:for<'a> fn(&'a mut KeyType, &'a mut ActCtx, &'a event::Event) -> Option<KeyFunc<Type, ActCtx, RetOk, RetErr>>) -> Self {
        Self{
            data_holder: data_holder,
            //keys: HashMap::default(),
            function: b,
            pd_0: PhantomData,
            pd_1: PhantomData,
            pd_2: PhantomData,
            pd_3: PhantomData,
            pd_4: PhantomData,
        }
    }
    
    pub(super) fn get_key_action(&mut self, ctx:&mut ActCtx, event:&event::Event) -> Option<KeyFunc<Type, ActCtx, RetOk, RetErr>> {
        
        (self.function)(&mut self.data_holder, ctx, event)
    }
}

/*
//TODO: Todo check this function
fn test_in<Type, KeyType, ActCtx, RetOk, RetErr>(holder:&mut KeyType, b:&mut ActCtx, c:&event::Event) -> Option<KeyFunc<Type, ActCtx, RetOk, RetErr>> {
    panic!();
}
*/
fn test_in<'a, Type>(holder:&mut SelectKeysDS<'a, Type>, b:&mut SelectActCtx<'a>, c:&event::Event) -> Option<KeyFunc<Type, SelectActCtx<'a>, SelOk, ()>> {
    panic!();
}

impl<Type> Keys<Type, SelectKeysDS<'_, Type>, SelectActCtx<'_>, SelOk, ()> {
    
    //fn test_in<Type, KeyType, ActCtx, RetOk, RetErr>(holder:&mut KeyType, b:&mut ActCtx, c:&event::Event) -> Option<KeyFunc<Type, ActCtx, RetOk, RetErr>> {
    fn test_in1<'a, 'b, 'c>(holder:&'a mut SelectKeysDS<'c, Type>, b:&'a mut SelectActCtx<'b>, c:&'a event::Event) -> Option<KeyFunc<Type, SelectActCtx<'b>, SelOk, ()>> {
        
        panic!();
    }
    
    
    pub fn default_keys() -> Self {
        
        let mut keys = Self::new(HashMap::default(), Self::test_in1);
        
        //TODO: try a unit case where you save a partial specific function from a generic function to see how it needs to be casted
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Char('k'),
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.data_holder.insert(key, Self::move_cursor_up), None);
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Char('j'),
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.data_holder.insert(key, Self::move_cursor_down), None);
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Enter,
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.data_holder.insert(key, Self::exit), None);
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.data_holder.insert(key, Self::abort), None);
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.data_holder.insert(key, Self::abort), None);
        keys
    }
    
    #[allow(dead_code)]
    fn exit(_:&[Type], ctx:&mut SelectActCtx) -> Result<SelOk, SelErr<()>> {
        let (_, index) = ctx;
        Ok(SelOk::Exit(**index))
    }
    
    #[allow(dead_code)]
    fn nope(_:&[Type], _:&mut SelectActCtx) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn move_cursor_down(_:&[Type], ctx:&mut SelectActCtx) -> Result<SelOk, SelErr<()>> {
        let (size, index) = ctx;
        if **index < *size-1 {
            **index += 1;
        }
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn move_cursor_up(_:&[Type], ctx:&mut SelectActCtx) -> Result<SelOk, SelErr<()>> {
        let (_, index) = ctx;
        if **index > 0 {
            **index -= 1;
        }
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn abort(_:&[Type], _:&mut SelectActCtx) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Abort)
    }
}

