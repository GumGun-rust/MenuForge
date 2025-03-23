use std::marker::PhantomData;
use std::io::Error as IOError;
use std::time::Duration;

use std::io::Write;
use std::io::stdout;

use crossterm::queue; 
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

mod keys;
pub use keys::Keys;
pub use keys::KeysTrait;
use keys::KeysTraitMut;

mod select;
pub use select::Select;
pub use select::SelOk;
pub use select::Configs as SelConf;
pub use select::ActCtx as SelectActCtx;
pub use select::KeysDS as SelectKeysDS;

/*
mod non_block;
pub use non_block::SelectNonBlock;
*/

const QUEUE_ERR:&'static str = "error in while setting stdout queue";
const PRINTLINE_ERR:&'static str = "error in while flushing";

//recomended to only modify the index field
//TODO: check the public level of this
pub type KeyFunc<Type, ActCtx, RetOk, RetErr> = fn(&[Type], &mut ActCtx)->Result<RetOk, SelErr<RetErr>>;
type KeyFuncMut<Type, ActCtx, RetOk, RetErr> = fn(&mut[Type], &mut ActCtx)->Result<RetOk, SelErr<RetErr>>;

pub enum SelErr<RetErr> {
    BaseErr(IOError),
    UserErr(RetErr),
}

pub enum RawSelResult<RetOk, RetErr> {
    Ok(RetOk),
    Err(RetErr),
    KeyNotFound,
}


#[derive(Derivative, Debug)]
#[derivative(Default)]
struct Fields {
    bottom: bool, //was menu started at the bottom
    #[derivative(Default(value="Point2{x:0,y:0}"))]
    window_measures: Point2<u16>,
    #[derivative(Default(value="Point2{x:0,y:0}"))]
    init_position: Point2<u16>,
}

#[derive(Default)]
pub struct RawSelect<Type, ActCtx, PrintCtx, RetOk, RetErr> {
    configs: RawConfigs,
    fields: Fields,
    pd_0: PhantomData<Type>,
    pd_1: PhantomData<ActCtx>,
    pd_2: PhantomData<PrintCtx>,
    pd_3: PhantomData<RetOk>,
    pd_4: PhantomData<RetErr>,
}

#[derive(Default)]
pub struct RawConfigs {
    pub table_size: u16,
}


impl<Type, ActCtx, PrintCtx, RetOk, RetErr> RawSelect<Type, ActCtx, PrintCtx, RetOk, RetErr> {
    
    //type PrintCbk = fn(u16, u16, &[Type], &mut PrintCtx)->Result<(), IOError>; 
    /* Inherent Associated types are unstable */
    
    pub fn new(configs: RawConfigs) -> Self {
        Self{
            configs,
            fields:Fields::default(),
            pd_0:PhantomData,
            pd_1:PhantomData,
            pd_2:PhantomData,
            pd_3:PhantomData,
            pd_4:PhantomData,
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
            queue!(
                stdout(), 
                ScrollUp(amount),
                MoveToPreviousLine(amount),
            )?;
            pos_y -= amount;
            *bottom = true;
        } 
        queue!(
            stdout(), 
            Hide
        )?;
        stdout().flush()?;
        
        *init_position = Point2{x:pos_x, y:pos_y};
        Ok(())
    }
    
    pub fn poll(&self) -> Result<bool, IOError> {
        poll(Duration::from_secs(0))
    }
    
    pub fn raw_prompt_mut<T:KeysTraitMut<Type, ActCtx, RetOk, RetErr>>(&mut self, keys:&mut T, list:&mut [Type], mut action_ctx:&mut ActCtx) -> RawSelResult<RetOk, SelErr<RetErr>> {
        
        let key = match read() {
            Ok(ok) => {ok}
            Err(err) => {return RawSelResult::Err(SelErr::BaseErr(err));}
        };
        
        match keys.get_key_action_mut(&mut action_ctx, &key) {
            Some(action) => {
                match action(list, &mut action_ctx) {
                    Ok(ok) => {RawSelResult::Ok(ok)}
                    Err(err) => {RawSelResult::Err(err)}
                }
            }
            None => {RawSelResult::KeyNotFound} 
        }
    }
    
    pub fn raw_prompt<T:KeysTrait<Type, ActCtx, RetOk, RetErr>>(&mut self, keys:&mut T, list:&[Type], mut action_ctx:&mut ActCtx) -> RawSelResult<RetOk, SelErr<RetErr>> {
        
        let key = match read() {
            Ok(ok) => {ok}
            Err(err) => {return RawSelResult::Err(SelErr::BaseErr(err));}
        };
        
        match keys.get_key_action(&mut action_ctx, &key) {
            Some(action) => {
                match action(&list, &mut action_ctx) {
                    Ok(ok) => {RawSelResult::Ok(ok)}
                    Err(err) => {RawSelResult::Err(err)}
                }
            }
            None => {RawSelResult::KeyNotFound} 
        }
    }
    
    pub fn end_prompt(&mut self) -> Result<(), IOError> {
        if self.fields.bottom {
            queue!(
                stdout(),
                ScrollUp(1),
            )?;
        } 
        queue!(
            stdout(),
            Show,
        )?;
        stdout().flush()?;
        disable_raw_mode()
    }
    
    pub fn print_line(&mut self, entries:&[Type], mut modi:PrintCtx, print_callback:fn(u16, u16, &[Type], &mut PrintCtx)->Result<(), IOError>) -> Result<(), IOError> {
        let Self{
            fields: Fields{
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
        queue!(
            stdout(), 
            MoveTo(init_position.x, init_position.y),
        )?;
        for line in 0..table_size{
            queue!(
                stdout(), 
                MoveToColumn(0),
            )?;
            print_callback(line, table_size, entries, &mut modi)?;
            queue!(
                stdout(), 
                MoveToNextLine(1),
            )?;
        }
        stdout().flush()
    }
}

