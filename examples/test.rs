// examples/hello.rs

extern crate menuforge;

fn main() {
    println!("Hello from an example!");
    println!("{:?}", menuforge::add(12, 12));
    
    let keys = menuforge::Select::gen_default_keys();
    let configs = menuforge::SelConfigs::default();
    
    let mut select = menuforge::Select::new(keys, configs);
    let mut a = [0i32];
    select.prompt(&mut a[..]);
    
}
