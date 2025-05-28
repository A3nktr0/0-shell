use crate::{
    closedir, lstat, opendir, readdir, stat, CStr, CString, DateTime, Duration, Local, S_IFDIR,
    UNIX_EPOCH,
};

// Ls command implementation "Low level system calls" version.
pub fn ls(args: &[&str]) {
    let mut show_hidden = false;
    let mut show_long = false;
    let mut show_type = false;

    // Parse options
    for arg in args {
        match *arg {
            "-a" => show_hidden = true,
            "-l" => show_long = true,
            "-F" => show_type = true,
            _ => {}
        }
    }

    // open directory
    let dir_path = CString::new(".").unwrap();
    let dir = unsafe { opendir(dir_path.as_ptr()) };
    if dir.is_null() {
        eprintln!("ls: Error opening directory");
        return;
    }

    let mut entries = Vec::new();
    let mut total_blocks = 0;

    unsafe {
        loop {
            let entry = readdir(dir);
            if entry.is_null() {
                break;
            }

            let entry = &*entry;
            let file_name = CStr::from_ptr(entry.d_name.as_ptr())
                .to_str()
                .unwrap()
                .to_string();

            // Skip hidden files if not showing hidden
            if !show_hidden && file_name.starts_with('.') {
                continue;
            }

            entries.push(file_name);
        }
        closedir(dir);
    }

    // Sort entries
    entries.sort_by(|a, b| a.cmp(b));

    if show_long {
        for file_name in &entries {
            let mut stat_buf: stat = unsafe { std::mem::zeroed() };
            let c_file_name = CString::new(file_name.clone()).unwrap();
            if unsafe { lstat(c_file_name.as_ptr(), &mut stat_buf) } != 0 {
                eprintln!("ls: Error getting file metadata for {}", file_name);
                continue;
            }
            total_blocks += stat_buf.st_blocks;
        }
        println!("total {}", total_blocks / 2);
    }

    for file_name in entries {
        if show_long {
            let mut stat_buf: stat = unsafe { std::mem::zeroed() };
            let c_file_name = CString::new(file_name.clone()).unwrap();
            if unsafe { lstat(c_file_name.as_ptr(), &mut stat_buf) } != 0 {
                eprintln!("ls: Error getting file metadata for {}", file_name);
                continue;
            }

            let file_type = if (stat_buf.st_mode & S_IFDIR) != 0 {
                "d"
            } else {
                "-"
            };
            let perm_str = format!(
                "{}{}{}{}{}{}{}{}{}",
                if stat_buf.st_mode & 0o400 != 0 {
                    "r"
                } else {
                    "-"
                },
                if stat_buf.st_mode & 0o200 != 0 {
                    "w"
                } else {
                    "-"
                },
                if stat_buf.st_mode & 0o100 != 0 {
                    "x"
                } else {
                    "-"
                },
                if stat_buf.st_mode & 0o040 != 0 {
                    "r"
                } else {
                    "-"
                },
                if stat_buf.st_mode & 0o020 != 0 {
                    "w"
                } else {
                    "-"
                },
                if stat_buf.st_mode & 0o010 != 0 {
                    "x"
                } else {
                    "-"
                },
                if stat_buf.st_mode & 0o004 != 0 {
                    "r"
                } else {
                    "-"
                },
                if stat_buf.st_mode & 0o002 != 0 {
                    "w"
                } else {
                    "-"
                },
                if stat_buf.st_mode & 0o001 != 0 {
                    "x"
                } else {
                    "-"
                }
            );

            let nlink = stat_buf.st_nlink;
            let uid = unsafe {
                libc::getpwuid(stat_buf.st_uid).as_ref().map_or_else(
                    || stat_buf.st_uid.to_string(),
                    |pw| CStr::from_ptr(pw.pw_name).to_str().unwrap().to_string(),
                )
            };
            let gid = unsafe {
                libc::getgrgid(stat_buf.st_gid).as_ref().map_or_else(
                    || stat_buf.st_gid.to_string(),
                    |gr| CStr::from_ptr(gr.gr_name).to_str().unwrap().to_string(),
                )
            };
            let size = stat_buf.st_size;
            let mtime = stat_buf.st_mtime;
            let datetime = DateTime::<Local>::from(UNIX_EPOCH + Duration::from_secs(mtime as u64));
            let formatted_time = datetime.format("%b %e %H:%M").to_string();

            let mut display_name = file_name.clone();
            if show_type {
                if (stat_buf.st_mode & S_IFDIR) != 0 {
                    display_name.push('/');
                }
            }

            println!(
                "{}{} {} {} {} {} {} {}",
                file_type, perm_str, nlink, uid, gid, size, formatted_time, display_name
            );
        } else {
            print!("{}", file_name);
            if show_type {
                let c_file_name = CString::new(file_name.clone()).unwrap();
                let mut stat_buf: stat = unsafe { std::mem::zeroed() };
                if unsafe { lstat(c_file_name.as_ptr(), &mut stat_buf) } != 0 {
                    eprintln!("ls: Error getting file metadata for {}", file_name);
                } else {
                    if (stat_buf.st_mode & S_IFDIR) != 0 {
                        print!("/");
                    }
                }
            }
            print!(" ");
        }
    }
    if !show_long {
        println!();
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// // Ls command implementation.
// pub fn ls(args: &[&str]) {
//     let mut show_hidden = false;
//     let mut show_long = false;
//     let mut show_type = false;

//     // Parse options
//     for arg in args {
//         match *arg {
//             "-a" => show_hidden = true,
//             "-l" => show_long = true,
//             "-F" => show_type = true,
//             _ => {}
//         }
//     }

//     // Read directory entries
//     let entries = match fs::read_dir(".") {
//         Ok(entries) => entries,
//         Err(e) => {
//             eprintln!("ls: {}", e);
//             return;
//         }
//     };

//     for entry in entries {
//         let entry = match entry {
//             Ok(entry) => entry,
//             Err(e) => {
//                 eprintln!("ls: {}", e);
//                 continue;
//             }
//         };

//         let file_name = entry.file_name();
//         let file_name = match file_name.to_str() {
//             Some(name) => name,
//             None => continue,
//         };

//         // Skip hidden files if not showing hidden
//         if !show_hidden && file_name.starts_with('.') {
//             continue;
//         }

//         // Show detailed information
//         if show_long {
//             let metadata = match entry.metadata() {
//                 Ok(metadata) => metadata,
//                 Err(e) => {
//                     eprintln!("ls: {}", e);
//                     continue;
//                 }
//             };

//             let permissions = metadata.permissions();
//             let file_type = if metadata.is_dir() { "d" } else { "-" };
//             let perm_str = format!(
//                 "{}{}{}{}{}{}{}{}{}",
//                 if permissions.mode() & 0o400 != 0 {
//                     "r"
//                 } else {
//                     "-"
//                 },
//                 if permissions.mode() & 0o200 != 0 {
//                     "w"
//                 } else {
//                     "-"
//                 },
//                 if permissions.mode() & 0o100 != 0 {
//                     "x"
//                 } else {
//                     "-"
//                 },
//                 if permissions.mode() & 0o040 != 0 {
//                     "r"
//                 } else {
//                     "-"
//                 },
//                 if permissions.mode() & 0o020 != 0 {
//                     "w"
//                 } else {
//                     "-"
//                 },
//                 if permissions.mode() & 0o010 != 0 {
//                     "x"
//                 } else {
//                     "-"
//                 },
//                 if permissions.mode() & 0o004 != 0 {
//                     "r"
//                 } else {
//                     "-"
//                 },
//                 if permissions.mode() & 0o002 != 0 {
//                     "w"
//                 } else {
//                     "-"
//                 },
//                 if permissions.mode() & 0o001 != 0 {
//                     "x"
//                 } else {
//                     "-"
//                 }
//             );

//             let nlink = metadata.nlink();
//             let uid = metadata.uid();
//             let gid = metadata.gid();
//             let size = metadata.size();
//             let mtime = metadata.mtime();
//             let datetime = DateTime::<Local>::from(UNIX_EPOCH + Duration::from_secs(mtime as u64));
//             let formatted_time = datetime.format("%b %e %H:%M").to_string();

//             println!(
//                 "{}{} {} {} {} {} {} {}",
//                 file_type, perm_str, nlink, uid, gid, size, formatted_time, file_name
//             );
//         } else {
//             print!("{}", file_name);
//             if show_type {
//                 match entry.file_type() {
//                     Ok(file_type) => {
//                         if file_type.is_dir() {
//                             print!("/");
//                         }
//                     }
//                     Err(e) => {
//                         eprintln!("ls: {}", e);
//                     }
//                 }
//             }
//             print!(" ");
//         }
//     }
//     if !show_long {
//         println!();
//     }
// }
