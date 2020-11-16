use std::io;
use std::process::Command;
use std::string;
use std::path::Path;
use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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

    let mut backup_dir = "/home/jeames8kin/BasicBackupTest";

    println!("Starting backup process...");

    let fun1 = run_fun!(rustc --version).unwrap();
    eprintln!("Installed Rust version: {}", fun1);

    let username1 = run_fun!(whoami).unwrap();

    println!("Aquiring root permissions...");

    if run_cmd! {
        sudo echo;
    }.is_err() {
        println!("Something went wrong: Unable to aquire root or dialogue was closed. Cannot proceed.");
        std::process::exit(1);
    }

    println!("Backup will include folders from: /home/{}/BasicBackupTest, do you want to proceed? (y/N)", username1);

    loop {

    let mut input1 = String::from("");

    io::stdin()
        .read_line(&mut input1)
        .expect("That isn't a valid answer");

    let mut input1_trimmed = input1.trim();

        match input1_trimmed {

            "yes" | "Yes" | "YES" | "y" | "Y" => {          // Defined Strings don't work for this? Like, at all? Says it can't access/find them.
                temp_dir_setup(backup_dir.to_string());
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

    fn temp_dir_setup(backup_dir:String) {

        let mut backup_dir1 = backup_dir;

        println!("Choose where to store the temporary directory (default is /tmp/LinuxBackup_Rust)");

        let mut directory = String::from("");

        io::stdin()
            .read_line(&mut directory)
            .expect("Test error");

        match directory.as_ref() {      //To compare a String, it needs to be in str rather than String, otherwise it just uses the first match arm. 
            "\n" => {
                directory = String::from("/tmp/LinuxBackup_Rust");
                make_dir(directory.trim().to_string(), backup_dir1);
            }

            _ => {
                make_dir(directory.trim().to_string(), backup_dir1);     //We need to copy/transfer/re-assign directory in a variable in make_dir, since once the program leaves the scope of the variable it basically destroys it.
            }

        }

    }

//-----------------------------------------------------------------------------

fn make_dir(directory:String, backup_dir:String) {      // This line acts as a function I can call and put a directory into its arguments, no extra code needed each folder.
    let directory1 = directory.as_str();
    let result = fs::create_dir(directory1);

    let mut choice1 = String::new();

    match result {

        Ok(_) => {
            println!("Created {}", directory1);
            let mut temp_dir_path = println!("{}", directory1);
            pv_check(directory1.to_string(), backup_dir)
        }

        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => {
            println!("{} already exists, clearing directory...", directory1);    
            let mut temp_dir_path = directory1;  

            if run_cmd! {
                rm -R ${temp_dir_path};
                mkdir ${temp_dir_path};
            }.is_err() {

                println!("Error: Unable to clear directory. Do you require elevated permissions for the chosen directory ({})?", directory1);

                io::stdin()
                    .read_line(&mut choice1)
                    .expect("Error: Input was either invalid or failed, please re-run the program!");
                    
                match choice1.as_str() {

                    "yes" | "Yes" | "YES" | "y" | "Y" => {

                        if run_cmd! {
                            sudo echo
                        }.is_err() {
                            if run_cmd! {
                                echo "Removing ${directory1}...";
                                sudo rm -R ${directory1};
                            }.is_err() {
                                println!("Error: Could not gain elevated priviledges! Cannot continue.")
                            }
                            
                        }

                    }

                    "no" | "No" | "NO" | "n" | "N" => {
                        println!("Aborted.");
                        std::process::exit(1);
                    }

                    _ => {

                    }

                }

            }

            pv_check(directory1.to_string(), backup_dir);
            
        }

        Err(ref e) if e.kind() == io::ErrorKind::PermissionDenied => {
            println!("Cannot create directory {}: Permission denied. Elevate priviledges?", directory1);
            io::stdin()
                .read_line(&mut choice1)
                .expect("Failed");

            let mut elevate_priviledges = String::from("");

            io::stdin()
                .read_line(&mut elevate_priviledges)
                .expect("That was invalid!");
        }

        Err(ref e) => {
            println!("Other error: {}", e);

        }

    }

}
    
//-----------------------------------------------------------------------------

    fn pv_check(temp_dir_path:String, backup_dir:String) {
        println!("Backing up {} (temp dir: {})", backup_dir, temp_dir_path);
        let mut system_time = SystemTime::now();
        
        let pv_check = run_fun!(pacman -Qqe | grep pv).unwrap();

        if pv_check == "pv" {
            println!("pv is already installed");
            has_pv(temp_dir_path, backup_dir);
        } else {
            if run_cmd! {
                sudo pacman -S pv;
            }.is_err() {
                println!("The backup can continue without pv, but the progress will not be displayed.");
                no_pv(temp_dir_path, backup_dir);
            }
        }
        
    } 

    fn has_pv(temp_dir_path:String, backup_dir:String) {

        if run_cmd! {
            echo "has pv";
            tar -czf - ${temp_dir_path}/backup.tar | pv > ${backup_dir};     // Questioning whether to .tar everything straight up or copy everything to the tmp folder and tar it there. 
        }.is_err() {
            println!("The archive failed?");
        }
    }

    fn no_pv(temp_dir_path:String, backup_dir:String) {

        let username1 = run_fun!(whoami).unwrap();

        if run_cmd! {
            echo "no pv";
//            tar -czf ${temp_dir_path}/backup.tar ${backup_dir};      // Questioning whether to .tar everything straight up or copy everything to the tmp folder and tar it there. 
            tar -czf /tmp/LinuxBackup_Rust/backup.tar /home/${username1}/rustBackupTest;
        }.is_err() {
            println!("The archive failed?");
        }
    }

    fn package_backup() {
        let package_list = run_fun!("pacman -Qqe | awk '{print $1}' | tr '\n' ' '").unwrap();
    }

    // dirSetup function bracket.

}