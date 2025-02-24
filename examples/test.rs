// examples/hello.rs

extern crate menu_ph;

fn main() {
    println!("Hello from an example!");
    println!("{:?}", menu_ph::add(12, 12));
    
    let keys = menu_ph::Select::gen_default_keys();
    let configs = menu_ph::SelConfigs::default();
    
    let mut select = menu_ph::Select::new(keys, configs);
    let mut a = [0i32];
    select.prompt(&mut a[..]);
    
}
