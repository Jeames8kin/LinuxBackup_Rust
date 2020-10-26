// Rust imports
use std::process::Command;
use std::string;
use std::io;

// Crates
use cmd_lib::run_cmd;
use cmd_lib::run_fun;

extern crate os_type;

pub fn basic_backup() {

    let scriptVerdict = false;         // Stays false until it reaches the end of the script successfully where it is true, otherwise will remain false.

    println!("basic backup test");

/*    let output = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start")
        .ok();
        
        let pacman_command = "pacman -Qqe | tr '\n' ' '";

        let pacman_output = Command::new(pacman_command)
        
        .spawn()
        .expect("pacman failed to reply.");
*/

    /*  let output = if cfg!(target_os = "windows") { //Checks if the host is Windows or Linux
            println!("Run this on an Arch derivative, otherwise this will not run!");
            std::process::exit(1);
        } else {
            println!("Checked host OS: Linux")
        }; */

    let _os = os_type::current_platform();
    match os_type::current_platform().os_type {
        os_type::OSType::Arch => {
            backup_start(); 
        } 
        os_type::OSType::Manjaro => {
            println!("Please note that using Manjaro may have slightly out of date packages compared to Arch and some things may not work.");
            backup_start();
        }
        _ => {
            println!("This operating system is invalid, please use Arch or Manjaro!");
            std::process::exit(1);
        }
    }
    
    // Worry about exiting the program if something fails later, more shit to do.

    fn backup_start() {
        println!("Beginning backup...");

        println!("Testing command capabilities...");
        let test1 = "ls ~/";
        let test2 = "LinuxBackup";

        let fun1 = run_fun!(rustc --version).unwrap(); // Sets output of command to variable.
        eprintln!("Your Rust version is {}", fun1);

        let username1 = run_fun!(whoami).unwrap();

        if run_cmd! {

            ls /home/${username1};                      // ${variable_here} is a way to use Rust defined variables in Shell.

        }.is_err() {

            println!("Something here doesn't seem right. ");

        }

        if run_cmd! {
            echo "Please enter your password to proceed.";
            sudo -s;

        }.is_err() {
            println!("Error 2: Program exited unexpectedly\n Please refer to the documentation for more about error codes!");
            std::process::exit(3);

        }

        println!("The current folders that will be backed up are:\n/home/jeames8kin/BasicBackupTest");
        println!("Are you sure you want to proceed?");
        let mut beginRestore = String::new();
        io::stdin() 
            .read_line(&mut beginRestore)
            .expect("That isn't a valid answer, answer yes/Yes/YES/y, or no/No/NO/n");


    }

} 



