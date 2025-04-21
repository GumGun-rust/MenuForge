fn main() {
    let mut keys = menuforge::Select::gen_default_keys();
    keys.ignore_extra_keys();
    let mut select = menuforge::Select::new(keys, 12);
    let mut options = ["a","b","c","d","e","f","g","h","i","j","k","1","2","3","4","5","6","7","8","9","0"];
    let holder = select.prompt(&mut options[..]).unwrap();
    match holder {
        Some(index) => {
            println!("{} ", index);
            println!("{} ", options[index]);
        }
        None => {
            println!("nothing");
        }
    }
    
}
