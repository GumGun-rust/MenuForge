use std::marker::PhantomData;
use std::collections::HashMap;
use std::io::Error as IOError;
use std::time::Duration;

use std::io::Write;
use std::io::stdout;

use crossterm::execute; 
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::terminal::ScrollUp;
use crossterm::queue; 
use crossterm::style::Print;
use crossterm::cursor::MoveTo;


use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use crossterm::event::read;
use crossterm::event::poll;
use crossterm::event;

mod raw;
//Am I over engineering?

type SelInputFunction<Type, RetOk, RetErr> = fn(&mut Type, &mut usize)->Result<RetOk, SelErr<RetErr>>;

pub enum SelOk {
    Ok,
    Exit,
}

pub enum SelErr<RetErr> {
    BaseErr(IOError),
    UserErr(RetErr),
}

pub enum SelResult<RetOk, RetErr> {
    Ok(RetOk),
    Err(RetErr),
    KeyNotFound,
}

//Options cant be updated in real time functions will block until the menu is completely clossed
//TODO: prompt in this should take a ref not a refmut
pub struct Select<Type> {
    configs: Configs,
    keys: Keys<Type, SelOk, ()>,
    inner: RawSelect<Type, SelOk,()>,
}

//For now this cant be updated in real time
pub struct SelectNonBlock<Type, RetOk, RetErr> {
    owner: Vec<Type>,
    keys: Keys<Type, RetOk, RetErr>,
    inner: RawSelect<Type, RetOk, RetErr>,
}

#[derive(Default)]
struct RawSelect<Type, RetOk, RetErr> {
    configs: RawConfigs,
    index: usize,
    pd_0: PhantomData<Type>,
    pd_1: PhantomData<RetOk>,
    pd_2: PhantomData<RetErr>,
}

pub struct Keys<Type, RetOk, RetErr> {
    keys: HashMap<event::Event,SelInputFunction<Type, RetOk, RetErr>>,
}

#[derive(Default)]
pub struct Configs {
    exit_on_new_key:bool,
}

#[derive(Default)]
struct RawConfigs {
    //exit_on_new_key:bool,
    //new_options:bool,
}


impl<Type, RetOk, RetErr> RawSelect<Type, RetOk, RetErr> {
    pub fn new(configs: RawConfigs) -> Self {
        Self{
            configs,
            index:0,
            pd_0:PhantomData,
            pd_1:PhantomData,
            pd_2:PhantomData,
        }
    }
    
    pub fn init_prompt(&mut self) -> Result<(), IOError> {
        enable_raw_mode();
        
        execute!(stdout(), ScrollUp(4))
    }
    
    pub fn poll(&self) -> Result<bool, IOError> {
        poll(Duration::from_secs(0))
    }
    
    pub fn raw_prompt(&mut self, keys:&Keys<Type, RetOk, RetErr>, list:&mut [Type]) -> SelResult<RetOk, SelErr<RetErr>> {
        let key = match read().map_err(|err|SelErr::BaseErr(err)){
            Ok(ok) => {ok}
            Err(err) => {return SelResult::Err(err);}
        };
        match keys.keys.get(&key) {
            Some(action) => {
                match action(&mut list[0], &mut self.index) {
                    Ok(ok) => {SelResult::Ok(ok)}
                    Err(_) => {
                        todo!("action returned error");
                    }
                }
            }
            None => {SelResult::KeyNotFound} }
    }
    
    pub fn end_prompt(&mut self) -> Result<(), IOError> {
        execute!(stdout());
        disable_raw_mode()
    }
    
    pub fn test_println(&mut self) {
        queue!(
            stdout(), 
            MoveTo(0, 0),
            Print("hola"),
        );
        
        stdout().flush();
    }

}

impl<Type, RetOk, RetErr> Default for Keys<Type, RetOk, RetErr> {
    fn default() -> Self {
        Self{
            keys:HashMap::new()
        }
    }
}

impl<Type> Keys<Type, SelOk, ()> {
    pub fn default_keys() -> Self {
        let mut keys = Self::default();
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Enter,
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.keys.insert(key, Self::move_cursor_down), None);
        let key = event::Event::Key(
            event::KeyEvent{
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                state: event::KeyEventState::NONE,
            }
        );
        assert_eq!(keys.keys.insert(key, Self::exit), None);
        keys
    }
    
    fn exit(_:&mut Type, _:&mut usize) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Exit)
    }
    
    fn move_cursor_down(_:&mut Type, index:&mut usize) -> Result<SelOk, SelErr<()>> {
        *index += 1;
        Ok(SelOk::Ok)
    }
    
    fn move_cursor_up(_:&mut Type, index:&mut usize) -> Result<SelOk, SelErr<()>> {
        *index -= 1;
        Ok(SelOk::Ok)
    }
}

impl<T> Select<T> {
    pub fn new(keys: Keys<T, SelOk, ()>, configs:Configs) -> Self {
        
        Self{
            configs,
            keys, 
            inner:RawSelect::<T, SelOk, ()>::new(RawConfigs::default())
        }
    }
    
    pub fn prompt(&mut self, list:&mut [T]) -> Result<usize, IOError> {
        for key in &self.keys.keys {
            println!("{:?}", key);
        }
        self.inner.init_prompt()?;
        loop{
            match self.inner.raw_prompt(&self.keys, list) {
                SelResult::Ok(ok) => {
                    match ok {
                        SelOk::Ok => {
                            self.inner.test_println();
                        }
                        SelOk::Exit => {
                            break;
                        }
                    }
                }
                SelResult::Err(_) => {
                    break;
                }
                SelResult::KeyNotFound => {
                    //write!(io::stdout(),"hola");
                    if self.configs.exit_on_new_key {
                        break;
                    }
                }
            }
        }
        self.inner.end_prompt()?;
        Ok(self.inner.index)
    }
    
    pub fn gen_default_keys() -> Keys<T, SelOk, ()> {
        Keys::<T, SelOk, ()>::default_keys()
    }
}

