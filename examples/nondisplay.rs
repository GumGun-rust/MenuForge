struct NonDisplay {
    field: String,
}

impl menuforge::SelectDisplay for NonDisplay {
    fn select_fmt(&self, f:&mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "NDisp  {} ", self.field)
    }
}

struct DisplayStruct {
    field: String,
}

impl std::fmt::Display for DisplayStruct {
    fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Disp  {} ", self.field)
    }
}

struct BothDisplay {
    field: String,
}

impl std::fmt::Display for BothDisplay {
    fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Disp  {} ", self.field)
    }
}

impl menuforge::SelectDisplay for BothDisplay {
    fn select_fmt(&self, f:&mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "NDisp  {} ", self.field)
    }
}

fn get_display_arr() -> Vec<DisplayStruct> {
    let mut holder:Vec<_> = Vec::new();
    for element in ["a","b","c","d","e","f","g","h","i","j","k","1","2","3","4","5","6","7","8","9","0"].iter() {
        
        holder.push(DisplayStruct{field:element.to_string()});
    }
    holder
}

fn get_non_display_arr() -> Vec<NonDisplay> {
    let mut holder:Vec<_> = Vec::new();
    for element in ["a","b","c","d","e","f","g","h","i","j","k","1","2","3","4","5","6","7","8","9","0"].iter() {
        
        holder.push(NonDisplay{field:element.to_string()});
    }
    holder
}

fn both_display_arr() -> Vec<BothDisplay> {
    let mut holder:Vec<_> = Vec::new();
    for element in ["a","b","c","d","e","f","g","h","i","j","k","1","2","3","4","5","6","7","8","9","0"].iter() {
        holder.push(BothDisplay{field:element.to_string()});
    }
    holder
}

fn main() {
    display_only_select();
    non_display_select();
    both_display_select();
}

fn both_display_select() {
    let keys = menuforge::Select::gen_default_keys();
    let configs = menuforge::SelConf::default();
    let mut select = menuforge::Select::new(keys, configs, 12);
    let mut a = get_display_arr();
    let holder = select.prompt(&mut a[..]).unwrap();
    match holder {
        Some(val) => {
            println!("{} ", val);
            println!("{}", a[val]);
        }
        None => {
            println!("nothing");
        }
    }
}

fn display_only_select() {
    let keys = menuforge::Select::gen_default_keys();
    let configs = menuforge::SelConf::default();
    let mut select = menuforge::Select::new(keys, configs, 12);
    let mut a = get_display_arr();
    let holder = select.prompt(&mut a[..]).unwrap();
    match holder {
        Some(val) => {
            println!("{} ", val);
            println!("{}", a[val]);
        }
        None => {
            println!("nothing");
        }
    }
}

fn non_display_select() {
    let keys = menuforge::Select::gen_default_keys();
    let configs = menuforge::SelConf::default();
    let mut select = menuforge::Select::new(keys, configs, 12);
    let mut a = get_non_display_arr();
    let holder = select.prompt(&mut a[..]).unwrap();
    match holder {
        Some(val) => {
            println!("{} ", val);
            println!("cant print since it is not display :)");
        }
        None => {
            println!("nothing");
        }
    }
}
