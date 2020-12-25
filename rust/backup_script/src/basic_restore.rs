use std::io;
use std::path::Path;

use cmd_lib::run_cmd;
use cmd_lib::run_fun;

extern crate os_type;
extern crate chrono;

pub fn br_check_os() {

    let os = os_type::current_platform();

    match os_type::current_platform().os_type {
        os_type::OSType::Arch => {
            file_verification();
        }
        os_type::OSType::Manjaro => {
            println!("Please note that Manjaro is not supported, if something breaks or doesn't work you are at fault! ");
            file_verification();
        }
        _ => {
            println!("This operating system is unsupported, please use Arch Linux!");
            std::process::exit(1);
        }

    }

}


pub fn file_verification() {
    

    println!("Elevating priviledges...");

    if run_cmd! {
        sudo echo;
    }.is_err() {
        println!("Failed to elevate priviledges. Try again?");

    loop {

            let mut input1 = String::new();

            io::stdin()
                .read_line(&mut input1)
                .expect("Failed");

            let mut input1_trim = input1.trim();

            match input1_trim {

                "yes" | "Yes" | "YES" | "y" | "Y" => {
                    if run_cmd! {
                        sudo echo;
                    }.is_err() {
                        println!("Aborted. Cannot proceed without root permissions.");
                        break;
                    }
                }

                "no" | "No" | "NO" | "n" | "N" => {
                    println!("Aborted.");
                    break;
                }

                _ => {

                }

            }

        }

    }
    
    loop {

    let mut input2 = String::new();

    println!("Enter the full directory/path to the backup file.");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed.");

    let mut input2_trim = input2.trim();

    

        println!("Checking {}...", input2_trim);
        let mut backup_dir_bool = String::new();
        let mut backup_dir_bool = Path::new(input2_trim).exists();

        if backup_dir_bool == false {

            println!("File and/or directory does not exist: {}", input2_trim);
        
        } else {
            
            restore(input2_trim.to_string());
            break;

        }

    }

    

    

}

pub fn restore(restore_dir:String) {

    println!("Copying file into temporary directory...");
    if run_cmd! {
        cp ${restore_dir} /tmp/LinuxBackup_Rust
    }.is_err() {
        println!("Failed for some reason?");
    }

}