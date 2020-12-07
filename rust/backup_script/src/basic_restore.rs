use std::io;

use cmd_lib::run_cmd;
use cmd_lib::run_fun;

extern crate os_type;
extern crate chrono;

pub fn br_check_os() {

    let os = os_type::current_platform();

    match os_type::current_platform().os_type {
        os_type::OSType::Arch => {
            basic_restore();
        }
        os_type::OSType::Manjaro => {
            println!("Please note that Manjaro is not supported, if something breaks or doesn't work you are at fault! ");
            basic_restore();
        }
        _ => {
            println!("This operating system is unsupported, please use Arch Linux!");
            std::process::exit(1);
        }

    }

}


pub fn basic_restore() {
    

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

            match input1.as_str() {

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
    
}
