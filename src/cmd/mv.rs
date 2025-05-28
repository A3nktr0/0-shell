use crate::{c_char, rename, CString, Path, OsStrExt};

// Mv command implementation "Low level system calls" version.
pub fn mv(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("Usage: mv <source> <destination>");
        return;
    }

    let src_path = Path::new(args[0]);
    let dest_path = Path::new(args[1]);

    let final_dest_path = if dest_path.is_dir() {
        dest_path.join(src_path.file_name().unwrap())
    } else {
        dest_path.to_path_buf()
    };

    let src_c = CString::new(src_path.as_os_str().as_bytes()).unwrap();
    let dest_c = CString::new(final_dest_path.as_os_str().as_bytes()).unwrap();

    let result = unsafe {
        rename(
            src_c.as_ptr() as *const c_char,
            dest_c.as_ptr() as *const c_char,
        )
    };

    if result != 0 {
        eprintln!("mv: {}: Error moving file or directory", args[0]);
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// // Mv command implementation.
// pub fn mv(args: &[&str]) {
//     if args.len() != 2 {
//         println!("Usage: mv <source> <destination>");
//         return;
//     }
//     if let Err(e) = fs::rename(args[0], args[1]) {
//         println!("mv: {}", e);
//     }
// }
