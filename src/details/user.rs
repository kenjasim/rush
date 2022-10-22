use std::mem;
use std::ptr;
use std::ffi::CStr;
use libc::{getpwuid_r, geteuid, gethostname};

// #[link(name = "c")]
// extern "C" {
//     fn geteuid() -> u32;
//     fn getegid() -> u32;
// }


pub fn get_user() -> Option<String>{

    unsafe{
        let mut result = ptr::null_mut();
        let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n < 0 => 512 as usize,
            n => n as usize,
        };
        let mut buf = Vec::with_capacity(amt);

        let mut passwd: libc::passwd = mem::zeroed();

        let uid = geteuid();

        match getpwuid_r(uid, &mut passwd, buf.as_mut_ptr(),
                                buf.capacity() as libc::size_t,
                                &mut result) {
            0 if !result.is_null() => {
                let ptr = passwd.pw_name as *const _;
                let username = CStr::from_ptr(ptr).to_str().unwrap().to_owned();
                Some(username)
            },
            _ => None
            }
    }

}

// pub fn get_hostname() -> Option<String>{

//     unsafe{

//         let len = 34;
//         let mut buf = std::vec::from_elem(len, 08);


//         match gethostname( 
//     }

// }



