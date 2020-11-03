use std::io;
use std::process::Command;
use std::string;
use std::path::Path;
use std::fs;

use cmd_lib::run_cmd;
use cmd_lib::run_fun;

extern crate os_type;

pub fn basic_backup() {
    let scriptVerdict = false;

    let os = os_type::current_platform();
    match os_type::current_platform().os_type {
        os_type::OSType::Arch => {
            testThing();
        }
        os_type::OSType::Manjaro => {
            println!("Please note that Manjaro is not supported, if something breaks or doesn't work you are at fault!");
            testThing();
        }
        _ => {
            println!("This operating system is unsupported, please use Arch Linux!");
            std::process::exit(1);
        }
    }


}

fn testThing() {

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
        println!("Something went wrong: Unable to aquire root or dialogue was closed.");
    }

    println!("Backup will include folders from: /home/{}/BasicBackupTest, do you want to proceed? (y/N)", username1);

    let mut input1 = String::from("");

    io::stdin()
        .read_line(&mut input1)
        .expect("That isn't a valid answer");

    let mut input1_trimmed = input1.trim();

    match input1_trimmed {

        "yes" | "Yes" | "YES" | "y" | "Y" => {          // Strings don't work for this? Like, at all? Says it can't access/find them.
            fileCopy();
        },
        "no" | "No" | "NO" | "n" | "N" => {
            println!("Aborted.");

        },
        _ => {
            println!("That wasn't a valid option!");
        }
    }

    fn fileCopy() {
        
        let username1 = run_fun!(whoami).unwrap();

        let tmpDir = "/tmp/LinuxBackup_Rust";

        if run_cmd! {
            sudo mkdir /tmp/LinuxBackup_Rust;
            ls /tmp/LinuxBackup_Rust;
            
            
        }.is_err() {
            println!("The folder {} exists. Would you like to delete it?", tmpDir);
        }


            let mut input2 = String::from("");

            io::stdin()
                .read_line(&mut input2)
                .expect("That isn't a valid answer");
        
            let mut input2_trimmed = input2.trim();
        
            match input2_trimmed {
        
                "yes" | "Yes" | "YES" | "y" | "Y" => {    
                          // Strings don't work for this? Like, at all? Says it can't access/find them.
                    if run_cmd! {
                        echo "Deleting folder...";
                        rm -R /tmp/LinuxBackup_Rust;
                        echo "Deleted.";
                    }.is_err() {
                        println!("Fucked if I know what happened");
                    }

                    fileCopy()

                },
                "no" | "No" | "NO" | "n" | "N" => {
                    println!("Aborted.");
        
                },
                _ => {
                    println!("That wasn't a valid option!");
                }
            }

    }

}