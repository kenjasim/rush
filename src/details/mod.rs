pub mod user;
pub mod pwd;




pub struct Details{
    pub user: String,
    pub hostname: String,
    pub cwd: String
}

pub fn get_details() -> Details{

    let dir = pwd::get_pwd();

    let d = Details{
        user: "kenan".to_string(),
        hostname: "fedora".to_string(),
        cwd: dir,
    };

    return d;
}



