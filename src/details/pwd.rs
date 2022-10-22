use std::env;

pub fn get_pwd() -> String{
    let cwd_path = env::current_dir()
        .unwrap();
    
    let cwd = cwd_path.to_str()
        .unwrap();

    return cwd.to_string();
}