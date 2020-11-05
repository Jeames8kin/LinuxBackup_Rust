use std::io;
use std::process::Command;
use std::string;
use std::path::Path;
use std::fs;

use cmd_lib::run_cmd;
use cmd_lib::run_fun;

extern crate os_type;

pub fn basic_backup() {
    let script_verdict = false;

    let os = os_type::current_platform();
    match os_type::current_platform().os_type {
        os_type::OSType::Arch => {
            test_thing();
        }
        os_type::OSType::Manjaro => {
            println!("Please note that Manjaro is not supported, if something breaks or doesn't work you are at fault!");
            test_thing();
        }
        _ => {
            println!("This operating system is unsupported, please use Arch Linux!");
            std::process::exit(1);
        }

    }

}

//-----------------------------------------------------------------------------

fn test_thing() {

    println!("Starting backup process...");

    let fun1 = run_fun!(rustc --version).unwrap();
    eprintln!("Installed Rust version: {}", fun1);

    let username1 = run_fun!(whoami).unwrap();

    if run_cmd! {
        ls /home/${username1};
    }.is_err() {
        println!("Something went wrong: Unable to list folders.");
    }

    println!("Aquiring root permissions...");

    if run_cmd! {
        sudo echo;
    }.is_err() {
        println!("Something went wrong: Unable to aquire root or dialogue was closed. Cannot proceed.");
    }

    println!("Backup will include folders from: /home/{}/BasicBackupTest, do you want to proceed? (y/N)", username1);

    let mut input1 = String::from("");

    io::stdin()
        .read_line(&mut input1)
        .expect("That isn't a valid answer");

    let mut input1_trimmed = input1.trim();


    loop {

        match input1_trimmed {

            "yes" | "Yes" | "YES" | "y" | "Y" => {          // Defined Strings don't work for this? Like, at all? Says it can't access/find them.
                dir_setup();
                break;
            },
            "no" | "No" | "NO" | "n" | "N" => {
                println!("Aborted.");
                break;

            },
            _ => {
                println!("That wasn't a valid option!");
            }
        }
    }

//-----------------------------------------------------------------------------

    fn dir_setup() {

        println!("Choose where to store the temporary directory (default is /tmp/LinuxBackup_Rust)\n>>>");

        let mut directory = String::from("");

        io::stdin()
            .read_line(&mut directory)
            .expect("Test error");

        match directory.as_ref() {
            "\n" => {
                directory = String::from("/tmp/LinuxBackup_Rust");
                make_dir(directory);
            }

            _ => {
                make_dir(directory.trim());
            }
        }


    }

        

//-----------------------------------------------------------------------------

    fn make_dir<T: AsRef<Path>>(path: T) {      // This line acts as a function I can call and put a directory into its arguments, no extra code needed each folder.
        let result = fs::create_dir(&path);
        match result {
            Ok(_) => {
                println!("Created {}", path.as_ref().display());
            }
            Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => {
                println!("{} already exists", path.as_ref().display());
            }
            Err(ref e) if e.kind() == io::ErrorKind::PermissionDenied => {
                println!("Cannot create directory {}: Permission denied. Elevate priviledges?", path.as_ref().display());
            }
            Err(ref e) => {
                println!("Other error: {}", e);
            }
        }
    }

    // dirSetup function bracket.

}