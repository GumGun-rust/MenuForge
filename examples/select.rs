
fn main() {
    let keys = menuforge::Select::gen_default_keys();
    let configs = menuforge::SelConf::default();
    let mut select = menuforge::Select::new(keys, configs, 12);
    let mut a = ["a","b","c","d","e","f","g","h","i","j","k","1","2","3","4","5","6","7","8","9","0"];
    let holder = select.prompt(&mut a[..]).unwrap();
    match holder {
        Some(val) => {
            println!("{} ", val);
            println!("{} ", a[val]);
        }
        None => {
            println!("nothing");
        }
    }
    
}
