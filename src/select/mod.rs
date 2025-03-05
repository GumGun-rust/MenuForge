use std::marker::PhantomData;
use std::collections::HashMap;
use std::io::Error as IOError;
use std::time::Duration;

use std::io::Write;
use std::io::stdout;

use crossterm::queue; 
use crossterm::execute; 
use crossterm::event;
use crossterm::event::read;
use crossterm::event::poll;
use crossterm::style::Print;
use crossterm::cursor::position;
use crossterm::cursor::MoveTo;
use crossterm::cursor::MoveToNextLine;
use crossterm::cursor::MoveToPreviousLine;
use crossterm::cursor::MoveToColumn;
use crossterm::cursor::Hide;
use crossterm::cursor::Show;

use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::terminal::ScrollUp;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::window_size;

use crossterm::style::SetColors; 
use crossterm::style::Color; 
use crossterm::style::Colors; 
use crossterm::style::ResetColor;

use mint::Point2;

use const_format::formatcp;

use derivative::Derivative;

use super::compat::symbols;

//mod raw;
//pub use raw::RawSelect;


const QUEUE_ERR:&'static str = "error in while setting stdout queue";
const PRINTLINE_ERR:&'static str = "error in while flushing";

//recomended to only modify the index field
//type SelInputFunction<Type, RetOk, RetErr> = fn(&Type, usize, &mut Fields)->Result<RetOk, SelErr<RetErr>>;

type SelInputFunctionMut<Type, RetOk, RetErr> = fn(&mut Type, usize, &mut Fields)->Result<RetOk, SelErr<RetErr>>;

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

//Options cant be updated in real time functions will block until the menu is completely clossed
//TODO: prompt in this should take a ref not a refmut
//
pub struct Select<Type> {
    configs: Configs,
    keys: Keys<Type, SelOk, ()>,
    inner: RawSelect<Type, SelOk,()>,
}

//For now this cant be updated in real time
pub struct SelectNonBlock<Type, RetOk, RetErr> {
    pub owner: Vec<Type>,
    pub keys: Keys<Type, RetOk, RetErr>,
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

#[derive(Derivative)]
#[derivative(Default)]
pub struct Keys<Type, RetOk, RetErr> {
    #[derivative(Default(bound=""))]
    keys: HashMap<event::Event, SelInputFunctionMut<Type, RetOk, RetErr>>,
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
    
    pub fn raw_prompt(&mut self, keys:&Keys<Type, RetOk, RetErr>, list:&mut [Type]) -> SelResult<RetOk, SelErr<RetErr>> {
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
        
        match keys.keys.get(&key) {
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

impl<Type> Keys<Type, SelOk, ()> {
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
    fn exit(_:&mut Type, _:usize, _:&mut Fields) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Exit)
    }
    
    #[allow(dead_code)]
    fn nope(_:&mut Type, _:usize, _:&mut Fields) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn move_cursor_down(_:&mut Type, size:usize, fields:&mut Fields) -> Result<SelOk, SelErr<()>> {
        if fields.index < size-1 {
            fields.index += 1;
        }
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn move_cursor_up(_:&mut Type, _:usize, fields:&mut Fields) -> Result<SelOk, SelErr<()>> {
        if fields.index > 0 {
            fields.index -= 1;
        }
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    fn abort(_:&mut Type, _:usize, _:&mut Fields) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Abort)
    }
}

impl<T:std::fmt::Display> Select<T> {
    
    pub fn new(keys: Keys<T, SelOk, ()>, configs:Configs) -> Self {
        
        let mut config_holder = RawConfigs::default();
        config_holder.table_size = 9;
        
        Self{
            configs,
            keys, 
            inner:RawSelect::<T, SelOk, ()>::new(config_holder)
        }
    }
    
    pub fn prompt(&mut self, list:&mut [T]) -> Result<Option<usize>, IOError> {
        self.inner.init_prompt()?;
        self.inner.print_line(list, Self::print_func)?;
        let ret = loop{
            match self.inner.raw_prompt(&self.keys, list) {
                SelResult::Ok(ok) => {
                    match ok {
                        SelOk::Ok => {
                            self.inner.print_line(list, Self::print_func).expect(PRINTLINE_ERR);
                        }
                        SelOk::Exit => {
                            break Some(self.inner.fields.index);
                        }
                        SelOk::Abort => {
                            break None;
                        }
                    }
                }
                SelResult::Err(_) => {
                    break None;
                }
                SelResult::KeyNotFound => {
                    if self.configs.exit_on_new_key {
                        break None;
                    }
                }
            }
        };
        self.inner.end_prompt()?;
        Ok(ret)
    }
    
    const UP_ARROW:&'static str = formatcp!(" {} ", symbols::UP_ARROW);
    const DOWN_ARROW:&'static str = formatcp!(" {} ", symbols::DOWN_ARROW);
    
    fn print_func(line:u16, menu_size:u16, index:usize, entries:&[T]) -> Result<(), IOError> {
        let half = menu_size/2/*+menu_size%2*/;
        let current_index;
        //index+usize::try_from(line).unwrap()
        let position_from_last = entries.len() - index ;
        
        if index < half.into() || entries.len() < (menu_size as usize) {
            current_index = usize::try_from(line).unwrap();
            if index == line.into() {
                Self::selected_line().expect(QUEUE_ERR);
            } else {
                if line == menu_size - 1 && entries.len() > menu_size.into() {
                    Self::bottom_line().expect(QUEUE_ERR);
                } else {
                    Self::empty_line().expect(QUEUE_ERR);
                }
            }
            
        } else if position_from_last <= half.into() {
            /* when cursor is at the bottom 
             * */
            current_index = entries.len() - menu_size as usize + line as usize;
            if line == menu_size - position_from_last as u16 {
                Self::selected_line().expect(QUEUE_ERR);
            } else {
                if line == 0 {
                    Self::top_line().expect(QUEUE_ERR);
                } else {
                    Self::empty_line().expect(QUEUE_ERR);
                }
            }
            
        }  else {
            current_index = index + line as usize - half as usize;
            if line == half {
                Self::selected_line().expect(QUEUE_ERR);
            } else {
                if line == 0 && index > half.into() {
                    Self::top_line().expect(QUEUE_ERR);
                } else if line == menu_size - 1 && position_from_last - 1 > half.into() {
                    Self::bottom_line().expect(QUEUE_ERR);
                } else {
                    Self::empty_line().expect(QUEUE_ERR);
                }
            }
        }
        if current_index < entries.len() {
            queue!(
                stdout(), 
                Print(&entries[current_index]),
                ResetColor,
                Clear(ClearType::UntilNewLine)
            ).expect(QUEUE_ERR);
        } else {
            queue!(
                stdout(), 
                ResetColor,
                Clear(ClearType::UntilNewLine)
            ).expect(QUEUE_ERR);
        }
        Ok(())
    }
    
    fn selected_line() -> Result<(), IOError> {
        queue!(
            stdout(), 
            Print(" > "),
            SetColors(Colors::new(Color::Blue, Color::Black))
        )
    }
    
    fn empty_line() -> Result<(), IOError> {
        queue!(
            stdout(), 
            Print("   ")
        )
    }
    
    fn bottom_line() -> Result<(), IOError> {
        queue!(
            stdout(), 
            Print(Self::DOWN_ARROW)
        )
    }
    
    fn top_line() -> Result<(), IOError> {
        queue!(
            stdout(), 
            Print(Self::UP_ARROW)
        )
    }
    
    pub fn gen_default_keys() -> Keys<T, SelOk, ()> {
        Keys::<T, SelOk, ()>::default_keys()
    }
    
}

