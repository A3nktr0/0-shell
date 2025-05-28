use crate::{c_void, close, open, read, write, CString, Path, O_RDONLY, STDOUT_FILENO};

// Cat command implementation "Low level system calls" version.
pub fn cat(args: &[&str]) {
    for arg in args {
        let path = Path::new(arg);
        let file_path = if path.extension().is_none() {
            path.with_extension("txt")
        } else {
            path.to_path_buf()
        };

        let c_arg = CString::new(file_path.to_string_lossy().to_string()).unwrap();
        let fd = unsafe { open(c_arg.as_ptr(), O_RDONLY) };
        if fd < 0 {
            eprintln!("cat: {}: No such file or directory", arg);
            continue;
        }

        let mut buffer = [0; 1024];
        loop {
            let bytes_read = unsafe { read(fd, buffer.as_mut_ptr() as *mut c_void, buffer.len()) };
            if bytes_read < 0 {
                eprintln!("cat: {}: Error reading file", arg);
                break;
            } else if bytes_read == 0 {
                break;
            } else {
                let bytes_written = unsafe {
                    write(
                        STDOUT_FILENO,
                        buffer.as_ptr() as *const c_void,
                        bytes_read as usize,
                    )
                };
                if bytes_written < 0 {
                    eprintln!("cat: {}: Error writing to stdout", arg);
                    break;
                }
            }
        }
        unsafe { close(fd) };
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// // Cat command implementation.
// pub fn cat(args: &[&str]) {
//     for arg in args {
//         let c_arg = CString::new(arg.to_string()).unwrap();
//         let fd = unsafe { open(c_arg.as_ptr(), O_RDONLY) };
//         if fd < 0 {
//             println!("cat: {}: Error opening file", arg);
//             continue;
//         }

//         let mut buffer = [0; 1024];
//         loop {
//             let bytes_read = unsafe { read(fd, buffer.as_mut_ptr() as *mut c_void, buffer.len()) };
//             if bytes_read < 0 {
//                 println!("cat: {}: Error reading file", arg);
//                 break;
//             } else if bytes_read == 0 {
//                 break;
//             } else {
//                 let output =
//                     unsafe { std::slice::from_raw_parts(buffer.as_ptr(), bytes_read as usize) };
//                 io::stdout().write_all(output).unwrap();
//             }
//         }

//         unsafe { close(fd) };
//     }
// }
