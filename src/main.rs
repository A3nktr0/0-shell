use libc::{c_char, getcwd, PATH_MAX};
use shell::{
    cmd::{cat::cat, cd::cd, cp::cp, echo::echo, ls::ls, mkdir::mkdir, mv::mv, pwd::pwd, rm::rm, clear::clear},
    Interface, ReadResult,
};
use std::ffi::CStr;

// 0-Shell implementation "Low level system calls" version.
fn main() {
    // create new interface for shell
    let interface = Interface::new("shell").unwrap();

    loop {
        // display current path location
        let mut buf = [0u8; PATH_MAX as usize];
        let ptr = buf.as_mut_ptr() as *mut c_char;

        let current_dir = unsafe {
            if getcwd(ptr, PATH_MAX as usize).is_null() {
                eprintln!("pwd: Error getting current directory");
                continue;
            } else {
                match CStr::from_ptr(ptr).to_str() {
                    Ok(path) => path,
                    Err(_) => {
                        eprintln!("pwd: Error converting path to string");
                        continue;
                    }
                }
            }
        };

        interface
            .set_prompt(&format!("{} $ ", current_dir))
            .unwrap();

        // Read input from the shell
        match interface.read_line() {
            Ok(ReadResult::Input(line)) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                let mut parts = input.split_whitespace();
                let command = parts.next().unwrap();
                let args: Vec<&str> = parts.collect();

                match command {
                    "echo" => echo(&args),
                    "cd" => cd(&args),
                    "ls" => ls(&args),
                    "pwd" => pwd(),
                    "cat" => cat(&args),
                    "cp" => cp(&args),
                    "rm" => rm(&args),
                    "mv" => mv(&args),
                    "mkdir" => mkdir(&args),
                    "clear" => clear(),
                    "exit" => break,
                    _ => {
                        println!("Command '{}' not found", command);
                    }
                }
            }
            Ok(ReadResult::Eof) => break,          // Crtl+D send EOF
            Ok(ReadResult::Signal(_)) => continue, // Crtl+C send signal, so we ignore it
            Err(error) => println!("Error reading input: {}", error),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// // 0-Shell implementation.
// fn main() {
//     // create new interface for shell
//     let interface = Interface::new("shell").unwrap();

//     loop {
//         // display current path location
//         interface
//             .set_prompt(&format!("{} $ ", env::current_dir().unwrap().display()))
//             .unwrap();

//         // Read input from the shell
//         match interface.read_line() {
//             Ok(ReadResult::Input(line)) => {
//                 let input = line.trim();
//                 if input.is_empty() {
//                     continue;
//                 }

//                 let mut parts = input.split_whitespace();
//                 let command = parts.next().unwrap();
//                 let args: Vec<&str> = parts.collect();

//                 match command {
//                     "echo" => echo(&args),
//                     "cd" => cd(&args),
//                     "ls" => ls(&args),
//                     "pwd" => pwd(),
//                     "cat" => cat(&args),
//                     "cp" => cp(&args),
//                     "rm" => rm(&args),
//                     "mv" => mv(&args),
//                     "mkdir" => mkdir(&args),
//                     "exit" => break,
//                     _ => {
//                         println!("Unknown command: {}", command);
//                     }
//                 }
//             }
//             Ok(ReadResult::Eof) => break,          // Crtl+D send EOF
//             Ok(ReadResult::Signal(_)) => continue, // Crtl+C send signal, so we ignore it
//             Err(error) => println!("Error reading input: {}", error),
//         }
//     }
// }
