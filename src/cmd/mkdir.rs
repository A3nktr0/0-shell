use crate::{CString, S_IRWXG, S_IRWXO, S_IRWXU};

// Mkdir command implementation "Low level system calls" version.
pub fn mkdir(args: &[&str]) {
    if args.len() != 1 {
        eprintln!("Usage: mkdir <directory>");
        return;
    }

    let path = CString::new(args[0]).unwrap();
    let result = unsafe { libc::mkdir(path.as_ptr(), S_IRWXU | S_IRWXG | S_IRWXO) };

    if result != 0 {
        eprintln!("mkdir: {}: Error creating directory", args[0])
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// // Mkdir command implementation.
// pub fn mkdir(args: &[&str]) {
//     if args.len() != 1 {
//         println!("Usage: mkdir <directory>");
//         return;
//     }

//     let path = Path::new(args[0]);
//     if let Err(e) = fs::create_dir(path) {
//         println!("mkdir: {}: {}", args[0], e);
//     }
// }
