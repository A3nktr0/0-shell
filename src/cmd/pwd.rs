use crate::{c_char, getcwd, write, CStr, PATH_MAX, STDOUT_FILENO};

// Pwd command implementation "Low level system calls" version.
pub fn pwd() {
    let mut buffer = vec![0u8; PATH_MAX as usize];
    let ptr = buffer.as_mut_ptr() as *mut c_char;

    unsafe {
        if getcwd(ptr, PATH_MAX as usize).is_null() {
            eprintln!("pwd: Error getting current directory");
        } else {
            match CStr::from_ptr(ptr).to_str() {
                Ok(_path) => {
                    let path = CStr::from_ptr(ptr).to_str().unwrap();
                    write(STDOUT_FILENO, path.as_ptr() as *const _, path.len());
                    write(STDOUT_FILENO, b"\n".as_ptr() as *const _, 1);
                }
                Err(_) => eprintln!("pwd: Error converting path to string"),
            }
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// // Pwd command implementation.
// pub fn pwd() {
//     match env::current_dir() {
//         Ok(path) => println!("{}", path.display()),
//         Err(e) => println!("pwd: {}", e),
//     }
// }
