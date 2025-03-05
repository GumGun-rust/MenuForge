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

//Options cant be updated in real time functions will block until the menu is completely clossed
//TODO: prompt in this should take a ref not a refmut
//
pub struct Select<Type> {
    configs: Configs,
    keys: Keys<Type, SelOk, ()>,
    inner: RawSelect<Type, SelOk,()>,
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
    
    pub fn prompt(&mut self, list:&[T]) -> Result<Option<usize>, IOError> {
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

