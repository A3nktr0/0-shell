use crate::{
    c_void, close, open, read, stat, write, CString, O_CREAT, O_RDONLY, O_TRUNC, O_WRONLY, S_IFDIR,
    S_IFMT, S_IRUSR, S_IWUSR,
};

// Check if the path is a directory "Low level system calls" version.
fn is_directory(path: &CString) -> bool {
    let mut stat_buf: stat = unsafe { std::mem::zeroed() };
    if unsafe { libc::stat(path.as_ptr(), &mut stat_buf) } == 0 {
        (stat_buf.st_mode & S_IFMT) == S_IFDIR
    } else {
        false
    }
}

// Get the file name from the path
fn get_file_name(path: &CString) -> CString {
    let path_str = path.to_str().unwrap();
    let file_name = path_str.rsplit('/').next().unwrap();
    CString::new(file_name).unwrap()
}

// Cp command implementation
pub fn cp(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("Usage: cp <source> <destination>");
        return;
    }

    let src_c = CString::new(args[0]).unwrap();
    let dest_c = CString::new(args[1]).unwrap();

    let final_dest_c = if is_directory(&dest_c) {
        let file_name = get_file_name(&src_c);
        let mut final_dest_path = dest_c.to_bytes().to_vec();
        final_dest_path.push(b'/');
        final_dest_path.extend_from_slice(file_name.to_bytes());
        CString::new(final_dest_path).unwrap()
    } else {
        dest_c.clone()
    };

    let src_fd = unsafe { open(src_c.as_ptr(), O_RDONLY) };
    if src_fd < 0 {
        eprintln!("cp: {}: Error opening source file\n", args[0]);
        return;
    }

    let dest_fd = unsafe {
        open(
            final_dest_c.as_ptr(),
            O_WRONLY | O_CREAT | O_TRUNC,
            S_IRUSR | S_IWUSR,
        )
    };
    if dest_fd < 0 {
        eprintln!("cp: {}: Error opening destination file\n", args[1]);
        unsafe { close(src_fd) };
        return;
    }

    let mut buffer = [0u8; 1024];
    loop {
        let bytes_read = unsafe { read(src_fd, buffer.as_mut_ptr() as *mut c_void, buffer.len()) };
        match bytes_read {
            n if n < 0 => {
                eprintln!("cp: {}: Error reading source file\n", args[0]);
                break;
            }
            0 => break,
            n => {
                let bytes_written =
                    unsafe { write(dest_fd, buffer.as_ptr() as *const c_void, n as usize) };
                if bytes_written < 0 {
                    eprintln!("cp: {}: Error writing destination file\n", args[1]);
                    break;
                }
            }
        }
    }

    unsafe {
        close(src_fd);
        close(dest_fd);
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// // Cp command implementation.
// pub fn cp(args: &[&str]) {
//     if args.len() != 2 {
//         println!("Usage: cp <source> <destination>");
//         return;
//     }

//     let src_path = Path::new(args[0]);
//     let dest_path = Path::new(args[1]);

//     // Check if destination is a directory
//     let final_dest_path = if dest_path.is_dir(){
//         dest_path.join(src_path.file_name().unwrap())
//     } else {
//         dest_path.to_path_buf()
//     };

//     let src_c = CString::new(args[0]).unwrap();
//     let dest_c = CString::new(final_dest_path.to_str().unwrap()).unwrap();

//     let src_fd = unsafe { open(src_c.as_ptr(), O_RDONLY) };
//     if src_fd < 0 {
//         println!("cp: {}: Error opening source file", args[0]);
//         return;
//     }

//     let dest_fd = unsafe {
//         open(
//             dest_c.as_ptr(),
//             O_WRONLY | O_CREAT | O_TRUNC,
//             S_IRUSR | S_IWUSR,
//         )
//     };
//     if dest_fd < 0 {
//         println!("cp: {}: Error opening destination file", args[1]);
//         unsafe { close(src_fd) };
//         return;
//     }

//     let mut buffer = [0, 1024];
//     loop {
//         let bytes_read = unsafe { read(src_fd, buffer.as_mut_ptr() as *mut c_void, buffer.len()) };
//         if bytes_read < 0 {
//             println!("cp: {}: Error reading source file", args[0]);
//             break;
//         } else if bytes_read == 0 {
//             break;
//         } else {
//             let bytes_written = unsafe {
//                 write(
//                     dest_fd,
//                     buffer.as_ptr() as *const c_void,
//                     bytes_read as usize,
//                 )
//             };
//             if bytes_written < 0 {
//                 println!("cp: {}: Error writing destination file", args[1]);
//                 break;
//             }
//         }
//     }

//     unsafe {
//         close(src_fd);
//         close(dest_fd);
//     }
// }
