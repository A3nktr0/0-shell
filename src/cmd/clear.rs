use crate::{write, CString, STDOUT_FILENO};

// Clear command implementation "Low level system calls" version.
pub fn clear() {
    let clear_sequence = if cfg!(target_os = "windows") {
        "\x1B[2J\x1B[1;1H"
    } else {
        "\x1B[2J\x1B[H"
    };

    let cstr = CString::new(clear_sequence).unwrap();
    unsafe {
        write(
            STDOUT_FILENO,
            cstr.as_ptr() as *const libc::c_void,
            clear_sequence.len(),
        );
    }
}

// // Clear command implementation.
// pub fn clear() {
//     if cfg!(target_os = "windows") {
//         // Windows escape sequence
//         print!("\x1B[2J\x1B[1;1H");
//     } else {
//         // Unix-like escape sequence
//         print!("\x1B[2J\x1B[H");
//     }
//     io::stdout().flush().expect("Failed to flush stdout");
// }
