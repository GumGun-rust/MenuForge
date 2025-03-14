use super::symbols;
use super::Keys;
use super::SelOk;
use super::SelResult;
use super::Configs;
use super::RawConfigs;
use super::RawSelect;
use super::QUEUE_ERR;
use super::PRINTLINE_ERR;

use std::io::Error as IOError;
use std::io::stdout;

use crossterm::queue; 
use crossterm::style::Print;
use crossterm::style::SetColors; 
use crossterm::style::Color; 
use crossterm::style::Colors; 
use crossterm::style::ResetColor;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;


use const_format::formatcp;

pub type ActCtx<'a> = (usize, &'a mut usize);
pub type PrintCtx = usize;

//Options cant be updated in real time functions will block until the menu is completely clossed
pub struct Select<'a, Type> {
    index: usize,
    configs: Configs,
    keys: Keys<Type, ActCtx<'a>, SelOk, ()>,
    inner: RawSelect<Type, ActCtx<'a>, usize, SelOk, ()>,
}



impl<'a, T:std::fmt::Display> Select<'a, T> {
    
    pub fn new(keys: Keys<T, ActCtx<'a>, SelOk, ()>, configs:Configs, table_size:u16) -> Self {
        
        let mut config_holder = RawConfigs::default();
        config_holder.table_size = table_size;
        
        Self{
            index: 0,
            configs,
            keys, 
            inner:RawSelect::<T, ActCtx<'a>, usize, SelOk, ()>::new(config_holder)
        }
    }
    
    pub fn prompt(&'a mut self, list:&[T]) -> Result<Option<usize>, IOError> {
        let mut ctx:ActCtx = (list.len(), &mut self.index);
        
        self.inner.init_prompt()?;
        self.inner.print_line(list, *ctx.1, Self::print_func)?;
        
        
        let ret = loop{
            match self.inner.raw_prompt(&self.keys, list, &mut ctx) {
                SelResult::Ok(ok) => {
                    match ok {
                        SelOk::Ok => {
                            self.inner.print_line(list, *ctx.1, Self::print_func).expect(PRINTLINE_ERR);
                        }
                        SelOk::Exit => {
                            break Some(10);
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
    
    
    fn print_func(line:u16, menu_size:u16, entries:&[T], print_ctx:&mut PrintCtx) -> Result<(), IOError> {
        let index = *print_ctx;
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
    
    pub fn gen_default_keys() -> Keys<T, ActCtx<'a>, SelOk, ()> {
        Keys::<T, ActCtx<'a>, SelOk, ()>::default_keys()
    }
    
}

