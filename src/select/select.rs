use super::symbols;
use super::Keys;
use super::RawSelResult;
use super::RawConfigs;
use super::RawSelect;
use super::QUEUE_ERR;
use super::PRINTLINE_ERR;
use super::KeyFunc;
use super::SelErr;

use std::io::Error as IOError;
use std::io::stdout;
use std::fmt::Display;
use std::collections::HashMap;

use crossterm::queue; 
use crossterm::style::Print;
use crossterm::style::Color; 
use crossterm::style::Colors; 
use crossterm::style::SetColors; 
use crossterm::style::ResetColor;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::event;


use const_format::formatcp;

pub type ActCtx<'a> = (usize, &'a mut usize);
pub type PrintCtx<'a> = (usize, Option<&'a String>);
pub type KeysDS<'a, Type> = HashMap<event::Event, KeyFunc<Type, ActCtx<'a>, SelOk, ()>>;

//extra options for setting up the menu
#[derive(Default)]
pub struct SelectConfig{
    title: Option<String>,
}

//Options cant be updated in real time functions will block until the menu is completely clossed
pub struct Select<'a, 'b, Type> {
    config: SelectConfig,
    index: usize,
    keys: Keys<Type, KeysDS<'b, Type>, ActCtx<'a>, SelOk, ()>,
    inner: RawSelect<Type, ActCtx<'a>, PrintCtx<'a>, SelOk, ()>,
}


pub enum SelOk {
    Ok,
    Exit(usize),
    Abort,
}

impl<'a, 'b, Type:Display> Select<'a, 'b, Type> {
    
    const UP_ARROW:&'static str = formatcp!(" {} ", symbols::UP_ARROW);
    const DOWN_ARROW:&'static str = formatcp!(" {} ", symbols::DOWN_ARROW);
    
    pub fn new_direct(keys: Keys<Type, KeysDS<'b, Type>, ActCtx<'a>, SelOk, ()>, config_arg:Option<SelectConfig>, table_size:u16) -> Self {
        let mut config_holder = RawConfigs::default();
        config_holder.table_size = table_size;
        let select_config = config_arg.unwrap_or_default();
        Self{
            config: select_config,
            index: 0,
            keys, 
            inner:RawSelect::<Type, ActCtx<'a>, PrintCtx, SelOk, ()>::new(config_holder)
        }
    }

    pub fn new(keys: Keys<Type, KeysDS<'b, Type>, ActCtx<'a>, SelOk, ()>, table_size:u16) -> Self {
        Self::new_direct(keys, None, table_size)
    }
    
    pub fn prompt(&'a mut self, list:&[Type]) -> Result<Option<usize>, IOError> {
        
        let mut ctx:ActCtx = (list.len(), &mut self.index);
        let mut print_ctx:PrintCtx = (*ctx.1, self.config.title.as_ref());
        
        self.inner.init_prompt()?;
        self.inner.print_buffer(list, print_ctx, Self::print_func)?;
        
        let ret = loop{
            match self.inner.raw_prompt(&mut self.keys, list, &mut ctx) {
                RawSelResult::Ok(ok) => {
                    match ok {
                        SelOk::Ok => {
                            print_ctx = (*ctx.1, self.config.title.as_ref());
                            self.inner.print_buffer(list, print_ctx, Self::print_func).expect(PRINTLINE_ERR);
                        }
                        SelOk::Exit(ret) => {
                            break Some(ret);
                        }
                        SelOk::Abort => {
                            break None;
                        }
                    }
                }
                RawSelResult::Err(_) => {
                    break None;
                }
                RawSelResult::KeyNotFound => {
                    break None;
                }
            }
        };
        self.inner.end_prompt()?;
        Ok(ret)
    }
    
    fn print_func(real_line:u16, real_menu_size:u16, entries:&[Type], print_ctx:&mut PrintCtx) -> Result<(), IOError> {

        let (line, menu_size) = match print_ctx.1{
            Some(title) => {
                if real_line == 0 {
                    queue!(
                        stdout(), 
                        Print(&title),
                        Clear(ClearType::UntilNewLine)
                    ).expect(QUEUE_ERR);
                    return Ok(());

                }
                let line = real_line - 1;
                let menu_size = real_menu_size - 1;
                (line, menu_size)
            }
            None => {
                (real_line, real_menu_size)
            }
        };


        if line >= menu_size {
            return Ok(());
        }

        let index = print_ctx.0;
        let half = menu_size/2;
        let pair_offset:usize = if menu_size%2==0 {1} else {0};
        let pair_complements:usize = if menu_size%2==0 {0} else {1};
        let current_index;
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
            
        } else if position_from_last+pair_offset <= half.into() {
            /* when cursor is at the bottom */
            current_index = entries.len() - menu_size as usize + line as usize;
            if line == menu_size - position_from_last as u16 {
                Self::selected_line().expect(QUEUE_ERR);
            } else {
                if line == 0 && entries.len() > menu_size.into() {
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
                } else if line == menu_size - 1 && position_from_last - pair_complements > half.into() {
                    Self::bottom_line().expect(QUEUE_ERR);
                } else {
                    Self::empty_line().expect(QUEUE_ERR);
                }
            }
        }
        queue!(
            stdout(), 
            Clear(ClearType::UntilNewLine)
        ).expect(QUEUE_ERR);
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
    
    pub fn gen_default_keys() -> Keys<Type, KeysDS<'a, Type>, ActCtx<'a>, SelOk, ()> {
        Keys::<Type, KeysDS<'a, Type>, ActCtx<'a>, SelOk, ()>::default_keys()
    }

    pub fn gen_default_configs() -> SelectConfig {
        SelectConfig::default()
    }

    #[allow(dead_code)]
    pub fn exit(_:&[Type], ctx:&mut ActCtx) -> Result<SelOk, SelErr<()>> {
        let (_, index) = ctx;
        Ok(SelOk::Exit(**index))
    }
    
    #[allow(dead_code)]
    pub fn nope(_:&[Type], _:&mut ActCtx) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    pub fn move_cursor_down(_:&[Type], ctx:&mut ActCtx) -> Result<SelOk, SelErr<()>> {
        let (size, index) = ctx;
        if **index < *size-1 {
            **index += 1;
        }
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    pub fn move_cursor_up(_:&[Type], ctx:&mut ActCtx) -> Result<SelOk, SelErr<()>> {
        let (_, index) = ctx;
        if **index > 0 {
            **index -= 1;
        }
        Ok(SelOk::Ok)
    }
    
    #[allow(dead_code)]
    pub fn abort(_:&[Type], _:&mut ActCtx) -> Result<SelOk, SelErr<()>> {
        Ok(SelOk::Abort)
    }

}


impl SelectConfig {
    pub fn set_title(&mut self, new_title:Option<String>) -> Option<String> {
        std::mem::replace(&mut self.title, new_title)
    }
}

