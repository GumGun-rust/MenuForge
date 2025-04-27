fn main() {
    let mut keys = menuforge::Select::gen_default_keys();
    keys.ignore_extra_keys();

    let configs = select.get_default_configs();
    let mut select = menuforge::Select::new(keys, Some(configs), 12);

    let mut options = ["a\t\t","bb\t\t","ccc\t\t","dddd\t","eeeee\t","ffffff\t","ggggggg","hhhhhhhh","iiiiiiii","j","k","1","2","3","4","5","6","7","8","9","0"];
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

fn simple_menu() {
    let mut keys = menuforge::Select::gen_default_keys();
    keys.ignore_extra_keys();
    let mut select = menuforge::Select::new(keys, None, 12);
    let mut options = ["a\t\t","bb\t\t","ccc\t\t","dddd\t","eeeee\t","ffffff\t","ggggggg","hhhhhhhh","iiiiiiii","j","k","1","2","3","4","5","6","7","8","9","0"];
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
    
