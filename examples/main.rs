extern crate menuforge;

fn main() {
    //println!("===================================");
    //println!("{}", a.len());
    let keys = menuforge::Select::gen_default_keys();
    let configs = menuforge::SelConfigs::default();
    let mut select = menuforge::Select::new(keys, configs, 9);
    let a = ["a","b","c","d","e","f","g","h","i","j","k","1","2","3","4","5","6","7","8","9","0"];
    //let mut a = ["a","b","c"];
    //let mut a = ["a","b","c","d","e","f","g"];
    //let mut a = ["a","b","c","d","e","f","g","h","i","j","k"];
    let holder = select.prompt(&a[..]).unwrap();
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
