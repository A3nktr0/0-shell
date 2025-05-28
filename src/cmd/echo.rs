use crate::{write, STDOUT_FILENO, CString};

// Echo command implementation "Low level system calls" version.
pub fn echo(args: &[&str]) {
    let mut args = args.join(" ");
    if args.starts_with('"') && args.ends_with('"') && args.len() > 1 {
        args = args[1..args.len() - 1].to_string();
    }

    match CString::new(args) {
        Ok(c_str) => unsafe {
            write(STDOUT_FILENO, c_str.as_ptr() as *const _, c_str.as_bytes().len());
            write(STDOUT_FILENO, b"\n".as_ptr() as *const _, 1);
        },
        Err(e) => eprintln!("Failed to create CString: {}", e),
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// // Echo command implementation.
// pub fn echo(args: &[&str]) {
//     let mut args = args.join(" ");
//     if args.starts_with('"') && args.ends_with('"') && args.len() > 1 {
//         args = args[1..args.len() - 1].to_string();
//     }
//     println!("{}", args);
// }
