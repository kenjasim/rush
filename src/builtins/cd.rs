use std::{path::Path, env};

pub fn cd(new_dir: &str){
    let root = Path::new(new_dir);
    if let Err(e) = env::set_current_dir(&root){
        eprintln!("{}", e)
    }
}