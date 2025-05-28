use crate::{getenv, CStr, CString};

// Cd command implementation "Low level system calls" version.
pub fn cd(args: &[&str]) {
    let home_dir = unsafe {
        let home_cstr = CString::new("HOME").unwrap();
        let home = getenv(home_cstr.as_ptr());
        if home.is_null() {
            eprintln!("cd: HOME environment variable is not set");
            return;
        }
        CStr::from_ptr(home).to_string_lossy().into_owned()
    };

    let target_dir = if args.is_empty() || args[0] == "~" {
        home_dir
    } else {
        args[0].to_string()
    };

    let path = CString::new(target_dir).unwrap();
    if unsafe { libc::chdir(path.as_ptr()) } != 0 {
        eprintln!("cd: Error changing directory");
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// // Cd command implementation.
// pub fn cd(args: &[&str]) {
//     let home_dir = match env::var("HOME") {
//         Ok(val) => val,
//         Err(_) => {
//             eprintln!("cd: HOME environment variable is not set");
//             return;
//         }
//     };

//     let target_dir = if args.is_empty() || args[0] == "~" {
//         home_dir
//     } else {
//         args[0].to_string()
//     };

//     let path = Path::new(&target_dir);
//     if let Err(e) = env::set_current_dir(&path) {
//         eprintln!("cd: {}", e);
//     }
// }
