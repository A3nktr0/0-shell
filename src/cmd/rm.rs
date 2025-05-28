use crate::{c_char, closedir, opendir, readdir, rmdir, unlink, CStr, CString, DT_DIR};

// Rm command implementation "Low level system calls" version.
pub fn rm(args: &[&str]) {
    let mut recursive = false;
    let mut target = "";

    // Parse options
    for arg in args {
        if *arg == "-r" {
            recursive = true;
        } else {
            target = arg;
        }
    }

    if target.is_empty() {
        eprintln!("Usage: rm [-r] <file>");
        return;
    }

    let path = CString::new(target).unwrap();
    // Remove directory recursively
    if recursive {
        if let Err(e) = remove_dir_recursive(&path) {
            eprintln!("rm: {}: {}", target, e);
        }
    } else {
        // Remove file
        let result = unsafe { unlink(path.as_ptr() as *const c_char) };
        if result != 0 {
            eprintln!("rm: {}: Error removing file", target);
        }
    }
}

// Remove directory recursively
fn remove_dir_recursive(path: &CString) -> Result<(), String> {
    unsafe {
        let dir = opendir(path.as_ptr());
        if dir.is_null() {
            return Err(format!(
                "Error opening directory: {}",
                path.to_str().unwrap()
            ));
        }

        loop {
            let entry = readdir(dir);
            if entry.is_null() {
                break;
            }

            let entry = &*entry;
            let name = CStr::from_ptr(entry.d_name.as_ptr()).to_str().unwrap();
            if name == "." || name == ".." {
                continue;
            }

            let mut entry_path = path.to_bytes().to_vec();
            entry_path.push(b'/');
            entry_path.extend_from_slice(name.as_bytes());
            let entry_path = CString::new(entry_path).unwrap();

            if entry.d_type == DT_DIR {
                remove_dir_recursive(&entry_path)?;
            } else {
                let result = unlink(entry_path.as_ptr() as *const c_char);
                if result != 0 {
                    closedir(dir);
                    return Err(format!(
                        "Error removing file: {}",
                        entry_path.to_str().unwrap()
                    ));
                }
            }
        }
        closedir(dir);
    }

    let result = unsafe { rmdir(path.as_ptr() as *const c_char) };
    if result != 0 {
        return Err(format!(
            "Error removing directory: {}",
            path.to_str().unwrap()
        ));
    }

    Ok(())
}


/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// // Rm command implementation.
// pub fn rm(args: &[&str]) {
//     let mut recursive = false;
//     let mut target = "";

//     // Parse options
//     for arg in args {
//         if *arg == "-r" {
//             recursive = true;
//         } else {
//             target = arg;
//         }
//     }

//     if target.is_empty() {
//         eprintln!("Usage: rm [-r] <file>");
//         return;
//     }

//     let path = Path::new(target);
//     if recursive {
//         // Remove directory recursively
//         if let Err(e) = fs::remove_dir_all(path) {
//             eprintln!("rm: {}: {}", target, e);
//         }
//     } else {
//         // Remove file
//         if let Err(e) = fs::remove_file(path) {
//             eprintln!("rm: {}: {}", target, e);
//         }
//     }
// }
