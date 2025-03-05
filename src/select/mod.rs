use std::marker::PhantomData;
use std::io::Error as IOError;
use std::time::Duration;

use std::io::Write;
use std::io::stdout;

use crossterm::queue; 
use crossterm::execute; 
use crossterm::event::read;
use crossterm::event::poll;
use crossterm::cursor::position;
use crossterm::cursor::MoveTo;
use crossterm::cursor::MoveToNextLine;
use crossterm::cursor::MoveToPreviousLine;
use crossterm::cursor::MoveToColumn;
use crossterm::cursor::Hide;
use crossterm::cursor::Show;

use crossterm::terminal::ScrollUp;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::window_size;

use mint::Point2;

use derivative::Derivative;

use super::compat::symbols;

mod select;
pub use select::Select;

mod keys;
pub use keys::KeysMut;
pub use keys::Keys;


const QUEUE_ERR:&'static str = "error in while setting stdout queue";
const PRINTLINE_ERR:&'static str = "error in while flushing";

//recomended to only modify the index field
type KeyFunc<Type, RetOk, RetErr> = fn(&Type, usize, &mut Fields)->Result<RetOk, SelErr<RetErr>>;
type KeyFuncMut<Type, RetOk, RetErr> = fn(&mut Type, usize, &mut Fields)->Result<RetOk, SelErr<RetErr>>;

pub enum SelOk {
    Ok,
    Exit,
    Abort,
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

//For now this cant be updated in real time
pub struct SelectNonBlock<Type, RetOk, RetErr> {
    pub owner: Vec<Type>,
    pub keys: KeysMut<Type, RetOk, RetErr>,
    pub inner: RawSelect<Type, RetOk, RetErr>,
}

#[derive(Derivative, Debug)]
#[derivative(Default)]
struct Fields {
    index: usize,
    bottom: bool, //was menu started at the bottom
    #[derivative(Default(value="Point2{x:0,y:0}"))]
    window_measures: Point2<u16>,
    #[derivative(Default(value="Point2{x:0,y:0}"))]
    init_position: Point2<u16>,
}

#[derive(Derivative, Debug)]
#[derivative(Default)]
pub struct Configs {
    #[derivative(Default(value="true"))]
    exit_on_new_key:bool,
}


#[derive(Default)]
pub struct RawSelect<Type, RetOk, RetErr> {
    configs: RawConfigs,
    fields: Fields,
    pd_0: PhantomData<Type>,
    pd_1: PhantomData<RetOk>,
    pd_2: PhantomData<RetErr>,
}

#[derive(Default)]
pub struct RawConfigs {
    table_size: u16,
    //exit_on_new_key:bool,
    //new_options:bool,
}


impl<Type, RetOk, RetErr> RawSelect<Type, RetOk, RetErr> {
    pub fn new(configs: RawConfigs) -> Self {
        Self{
            configs,
            fields:Fields::default(),
            pd_0:PhantomData,
            pd_1:PhantomData,
            pd_2:PhantomData,
        }
    }
    
    pub fn init_prompt(&mut self) -> Result<(), IOError> {
        enable_raw_mode()?;
        stdout().flush()?;
        let Self{
            configs: RawConfigs{
                table_size,
            },
            fields: Fields{
                window_measures,
                init_position,
                bottom,
                ..
            },
            ..
        } = self;
        
        let table_size = *table_size;
        
        let (pos_x, mut pos_y) = position()?;
        let window = window_size()?;
        
        *window_measures = Point2{x:window.columns, y:window.rows};
        
        let space_left = window_measures.y - pos_y -1;
        if space_left < table_size {
            let amount = table_size - space_left - 1;
            execute!(stdout(), 
                ScrollUp(amount),
                MoveToPreviousLine(amount),
                Hide
            )?;
            pos_y -= amount;
            *bottom = true;
        } 
        
        *init_position = Point2{x:pos_x, y:pos_y};
        Ok(())
    }
    
    pub fn poll(&self) -> Result<bool, IOError> {
        poll(Duration::from_secs(0))
    }
    
    pub fn raw_prompt_mut(&mut self, keys:&KeysMut<Type, RetOk, RetErr>, list:&mut [Type]) -> SelResult<RetOk, SelErr<RetErr>> {
        let key = match read().map_err(|err|SelErr::BaseErr(err)){
            Ok(ok) => {ok}
            Err(err) => {return SelResult::Err(err);}
        };
        let Self{
            fields: Fields{
                index,
                ..
            },
            ..
        }= self;
        let len = list.len();
        
        match keys.keys_get().get(&key) {
            Some(action) => {
                match action(&mut list[*index], len, &mut self.fields) {
                    Ok(ok) => {SelResult::Ok(ok)}
                    Err(_) => {
                        todo!("action returned error");
                    }
                }
            }
            None => {SelResult::KeyNotFound} }
    }
    
    pub fn raw_prompt(&mut self, keys:&Keys<Type, RetOk, RetErr>, list:&[Type]) -> SelResult<RetOk, SelErr<RetErr>> {
        let key = match read().map_err(|err|SelErr::BaseErr(err)){
            Ok(ok) => {ok}
            Err(err) => {return SelResult::Err(err);}
        };
        let Self{
            fields: Fields{
                index,
                ..
            },
            ..
        }= self;
        let len = list.len();
        match keys.keys_get().get(&key) {
            Some(action) => {
                match action(&list[*index], len, &mut self.fields) {
                    Ok(ok) => {SelResult::Ok(ok)}
                    Err(_) => {
                        todo!("action returned error");
                    }
                }
            }
            None => {SelResult::KeyNotFound} }
    }
    
    pub fn end_prompt(&mut self) -> Result<(), IOError> {
        if self.fields.bottom {
            execute!(
                stdout(),
                Show,
                ScrollUp(1),
            )?;
        }
        disable_raw_mode()
    }
    
    pub fn print_line(&mut self, entries:&[Type], print_callback:fn(u16, u16, usize, &[Type])->Result<(), IOError>) -> Result<(), IOError> {
        let Self{
            fields: Fields{
                index,
                init_position,
                ..
            },
            configs: RawConfigs{
                table_size,
                ..
            },
            ..
        } = self;
        
        let table_size = *table_size;
        let index = *index;
        queue!(
            stdout(), 
            MoveTo(init_position.x, init_position.y),
        )?;
        
        for line in 0..table_size{
            queue!(
                stdout(), 
                MoveToColumn(0),
            )?;
            print_callback(line, table_size, index, entries)?;
            queue!(
                stdout(), 
                MoveToNextLine(1),
            )?;
        }
        stdout().flush()
    }

}
